import sys
from typing import List

from matrix_shader_runner import *
from coop_matrix_defs import (
    MATRIX_USAGE_SHORT_NAME,
    SUPPORTED_CFGS_SM75,
    SUPPORTED_CFGS_SM86,
    VK_TYPE_TO_BYTE_SIZE,
)


def lea_hi(a, b, c, shift):
    if shift == 0:
        high = 0
    else:
        high = a >> (32 - shift)

    high = high | c << shift

    return high + b


RZ = 0
mat_store_addr = 0

LOAD_PER_MATRIX_PER_THREAD = 2
LOAD_PER_MATRIX_PER_THREAD_U8 = 4

HW_MATRIX_88 = 8 * 8
THREAD_COUNT = 32


def compute_16x8x8_target_by_lane_id(
    lane_id: int, idx: int, is_type_a: bool
) -> Tuple[int, int]:
    group_id = lane_id >> 2
    thread_id_in_group = lane_id % 4
    row = 0
    col = 0

    if is_type_a:
        if idx == 0 or idx == 1:
            row = group_id
        elif idx == 2 or idx == 3:
            row = group_id + 8
        col = thread_id_in_group * 2 + (idx & 1)
    else:
        if idx == 0 or idx == 2:
            row = group_id
        elif idx == 1 or idx == 3:
            row = group_id + 8

        if idx == 0 or idx == 1:
            col = thread_id_in_group
        elif idx == 2 or idx == 3:
            col = thread_id_in_group + 4

    return (row, col)


def compute_mat_offset_new(
    row: int,
    column: int,
    stride: int,
    element: int,
    byte_size: int,
    lane_id: int,
    hw_idx: int,
    is_colmn_major: bool,
) -> int:
    mat_store_base_addr = mat_store_addr + element * byte_size

    element_count = (row * column) // 32

    value_per_32_reg = 4 // byte_size

    (target_row, target_col) = compute_16x8x8_target_by_lane_id(
        lane_id, hw_idx % 4, not is_colmn_major
    )
    if is_colmn_major:
        # TODO: Broken
        major_offset = target_col * stride
        minor_offset = target_row
        print((target_row, target_col))
    else:
        major_offset = target_row * stride
        minor_offset = target_col

    offset = (major_offset + minor_offset) * byte_size

    if is_colmn_major:
        # TODO: Broken
        offset += (hw_idx // 4) * 8 * byte_size
    else:
        offset += (hw_idx // 4) * 8 * byte_size

    return mat_store_base_addr + offset


# Some formats on SM75 are still broken:
# - 8x8 (Column Major, 4b per element)
# - 8x32 (Row Major, 1b per element)
def compute_mat_offset(
    row: int,
    column: int,
    stride: int,
    element: int,
    byte_size: int,
    lane_id: int,
    hw_idx: int,
    is_colmn_major: bool,
) -> int:
    major_idx = lane_id // 0x4
    minor_idx = lane_id % 0x4

    if byte_size == 1:
        load_per_matrix_per_thread = LOAD_PER_MATRIX_PER_THREAD_U8
    else:
        load_per_matrix_per_thread = LOAD_PER_MATRIX_PER_THREAD

    major_offset = major_idx
    minor_offset = minor_idx * load_per_matrix_per_thread
    mat_store_base_addr = mat_store_addr + element * byte_size

    # 8x8 F16/U32/S32 (Row and Column Major)
    # 16x8 F16/F32/U32/S32 (Row and Column Major)
    # 16x16 F16/F32/U32/S32 (Row and Column Major)
    # 16x32 U8/S8 (Row and Column Major)
    # 32x16 U8/S8 (Row and Column Major)
    # 32x8 U8/S8 (Row and Column Major)
    is_row_major_32x8_u8 = (
        byte_size == 1 and not is_colmn_major and row == 32 and column == 8
    )
    is_row_major_32x16_u8 = (
        byte_size == 1 and not is_colmn_major and row == 32 and column == 16
    )
    is_colmn_major_8x32_u8 = (
        byte_size == 1 and is_colmn_major and row == 32 and column == 8
    )
    is_colmn_major_32x8_u8 = (
        byte_size == 1 and is_colmn_major and row == 8 and column == 32
    )

    idx_a = hw_idx // load_per_matrix_per_thread
    idx_b = hw_idx % load_per_matrix_per_thread

    if is_row_major_32x8_u8:
        idx_a = hw_idx // 2
        idx_b = hw_idx % 2

    if is_row_major_32x8_u8 or is_row_major_32x16_u8:
        tmp = idx_a
        idx_a = idx_b
        idx_b = tmp

    minor_offset += idx_b

    if is_row_major_32x8_u8 or is_colmn_major_32x8_u8 or is_colmn_major_8x32_u8:
        if idx_a >= 1:
            minor_offset += load_per_matrix_per_thread * 0x4
    else:
        major_offset += (idx_a % 2) * 0x8
        if idx_a >= 2:
            minor_offset += load_per_matrix_per_thread * 0x4

    # 32x8 U8/S8 Row Major
    # 32x16 U8/S8 Row Major
    # 8x8 F16/U32/S32 Column Major
    # 16x8 F16/U32/S32 Column Major
    # 16x16 F16/U32/S32 Column Major
    # 16x32 U8/S8 Column Major
    # 8x32 U8/S8 Column Major
    if (
        (
            is_colmn_major
            and not (col == 32 and (row == 8 or row == 16))
            and not (row == 32 and (col == 8 or col == 16))
        )
        or is_row_major_32x8_u8
        or is_row_major_32x16_u8
    ):
        tmp = major_offset
        major_offset = minor_offset
        minor_offset = tmp

    return mat_store_base_addr + (major_offset * stride + minor_offset) * byte_size


# XXX: Seems quite OFF?? should be storing u8 not u16...?
def compute_row_major_mat16x16_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    # c[0x0][0x50] = stride buffer
    R6 = stride
    # c[0x0][0x40] = element buffer
    R2 = element

    # SHF.R.U32.HI R1, RZ, 0x2, R7 ;
    R1 = lane_id >> 0x2

    # LOP3.LUT R7, R7, 0x3, RZ, 0xc0, !PT ;
    R7 = lane_id & 0x3

    # IMAD R8, R1, R6, RZ ;
    R8 = R1 * R6 + RZ

    # IMAD R7, R7, 0x4, R8 ;
    R7 = R7 * 0x4 + R8

    # MOV R0, R2 ;
    R0 = R2

    # IMAD.MOV.U32 R1, RZ, RZ, RZ ;
    R1 = RZ

    # IADD3 R4, R7, 0x10, RZ ;
    R4 = R7 + 0x10 + RZ

    # IMAD R6, R6, 0x8, R7 ;
    R6 = R6 * 0x8 + R7

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    # c[0x0][0x30] = mat store addr
    R8 = R0 + mat_store_addr + RZ

    # IADD3 R1, R6.reuse, 0x10, RZ ;
    R1 = R6 + 0x10 + RZ

    # IADD3 R0, P0, R7, R8.reuse, RZ ;
    R0 = R7 + R8 + RZ

    # IADD3 R2, P1, R6, R8.reuse, RZ ;
    R2 = R6 + R8 + RZ

    # IADD3 R6, P3, R1, R8.reuse, RZ ;
    R6 = R1 + R8 + RZ

    # IADD3 R4, P2, R4, R8, RZ ;
    R4 = R4 + R8 + RZ

    res.append(R0 + 0)
    res.append(R0 + 1)
    res.append(R2 + 0)
    res.append(R2 + 1)
    res.append(R4 + 0)
    res.append(R4 + 1)
    res.append(R6 + 0)
    res.append(R6 + 1)

    return res


def compute_column_major_mat16x16_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    # c[0x0][0x50] = stride buffer
    R11 = stride
    # c[0x0][0x40] = element buffer
    R2 = element

    # S2R R7, SR_LANEID ;
    R7 = lane_id

    # IMAD.SHL.U32 R0, R7, 0x4, RZ ;
    R0 = R7 * 0x4

    # LOP3.LUT R0, R0, 0xc, RZ, 0xe2, !PT ;
    R0 = R0 & 0xC

    # IMAD R6, R0, R11, RZ ;
    R6 = R0 * R11

    # LEA.HI R6, R7, R6, RZ, 0x1e ;
    R6 = lea_hi(R7, R6, RZ, 0x1E)

    # IMAD.MOV.U32 R0, RZ, RZ, R2 ;
    R0 = R2

    # IADD3 R7, R6, 0x8, RZ ;
    R7 = R6 + 0x8 + RZ

    # IMAD.MOV.U32 R1, RZ, RZ, RZ ;
    R1 = RZ

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    R8 = R0 + mat_store_addr + RZ

    # IADD3 R0, P1, R6, R8.reuse, RZ ;
    R0 = R6 + R8 + RZ

    # IADD3 R2, P0, R7, R8, RZ ;
    R2 = R7 + R8 + RZ

    # IADD3 R4, P1, R11.reuse, R0, RZ ;
    R4 = R11 + R0 + RZ

    res.append(R0)

    # IADD3 R6, P0, R11, R2, RZ ;
    R6 = R11 + R2 + RZ

    res.append(R2)

    # IADD3 R8, P1, R11.reuse, R4, RZ ;
    R8 = R11 + R4 + RZ

    # IADD3 R0, P0, R11, R6, RZ ;
    R0 = R11 + R6 + RZ

    res.append(R4)
    res.append(R6)

    # IADD3 R10, P1, R11, R8, RZ ;
    R10 = R11 + R8 + RZ

    # IADD3 R2, P0, R11, R0, RZ ;
    R2 = R11 + R0 + RZ

    res.append(R8)
    res.append(R0)
    res.append(R10)
    res.append(R2)

    return res


def compute_column_major_mat16x16_offset_f32_simplified(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()
    byte_size = 4

    major_idx = lane_id // 0x4
    minor_idx = lane_id % 0x4

    if byte_size == 1:
        load_per_matrix_per_thread = LOAD_PER_MATRIX_PER_THREAD_U8
    else:
        load_per_matrix_per_thread = LOAD_PER_MATRIX_PER_THREAD

    major_offset = major_idx
    minor_offset = minor_idx * load_per_matrix_per_thread
    mat_store_base_addr = mat_store_addr + element * byte_size

    # @P1 IMAD.SHL.U32 R15, R4, 0x4, RZ ;
    # R15 = stride * byte_size + RZ

    # IMAD R6, R7, R15, R6 ;
    # R6 = (minor_offset * stride + major_offset) * byte_size

    # IMAD R8, R15, 0x8, R6 ;
    # R8 = R15 * 0x8 + R6

    # IADD3 R12, R8, 0x20, RZ ;
    # R12 = R8 + 0x20 + RZ

    mat0_major_offset = major_offset
    mat0_minor_offset = minor_offset

    mat1_major_offset = major_offset
    mat1_minor_offset = minor_offset + 1

    mat2_major_offset = major_offset + 0x8
    mat2_minor_offset = minor_offset

    mat3_major_offset = major_offset + 0x8
    mat3_minor_offset = minor_offset + 1

    mat4_major_offset = major_offset
    mat4_minor_offset = minor_offset + 0x8

    mat5_major_offset = major_offset
    mat5_minor_offset = minor_offset + 0x8 + 1

    mat6_major_offset = major_offset + 0x8
    mat6_minor_offset = minor_offset + 0x8

    mat7_major_offset = major_offset + 0x8
    mat7_minor_offset = minor_offset + 0x8 + 1

    tmp = mat0_major_offset
    mat0_major_offset = mat0_minor_offset
    mat0_minor_offset = tmp
    mat0 = (
        mat_store_base_addr
        + (mat0_major_offset * stride + mat0_minor_offset) * byte_size
    )

    tmp = mat1_major_offset
    mat1_major_offset = mat1_minor_offset
    mat1_minor_offset = tmp
    mat1 = (
        mat_store_base_addr
        + (mat1_major_offset * stride + mat1_minor_offset) * byte_size
    )

    tmp = mat2_major_offset
    mat2_major_offset = mat2_minor_offset
    mat2_minor_offset = tmp
    mat2 = (
        mat_store_base_addr
        + (mat2_major_offset * stride + mat2_minor_offset) * byte_size
    )

    tmp = mat3_major_offset
    mat3_major_offset = mat3_minor_offset
    mat3_minor_offset = tmp
    mat3 = (
        mat_store_base_addr
        + (mat3_major_offset * stride + mat3_minor_offset) * byte_size
    )

    tmp = mat4_major_offset
    mat4_major_offset = mat4_minor_offset
    mat4_minor_offset = tmp
    mat4 = (
        mat_store_base_addr
        + (mat4_major_offset * stride + mat4_minor_offset) * byte_size
    )

    tmp = mat5_major_offset
    mat5_major_offset = mat5_minor_offset
    mat5_minor_offset = tmp
    mat5 = (
        mat_store_base_addr
        + (mat5_major_offset * stride + mat5_minor_offset) * byte_size
    )

    tmp = mat6_major_offset
    mat6_major_offset = mat6_minor_offset
    mat6_minor_offset = tmp
    mat6 = (
        mat_store_base_addr
        + (mat6_major_offset * stride + mat6_minor_offset) * byte_size
    )

    tmp = mat7_major_offset
    mat7_major_offset = mat7_minor_offset
    mat7_minor_offset = tmp
    mat7 = (
        mat_store_base_addr
        + (mat7_major_offset * stride + mat7_minor_offset) * byte_size
    )

    # IADD3 R0, P3, R6, R13.reuse, RZ ;
    # R0 = R6 + mat_store_base_addr

    # IADD3 R2, P3, R15, R0, RZ ;
    # R2 = mat0 + R15 + RZ

    # IADD3 R4, P0, R4, R13.reuse, RZ ;
    # R4 = R4 + mat_store_base_addr + RZ

    # IADD3 R6, P0, R15.reuse, R4, RZ ;
    # R6 = R15 + R4 + RZ

    # IADD3 R8, P1, R8, R13.reuse, RZ ;
    # R8 = R8 + mat_store_base_addr + RZ

    # IADD3 R10, P1, R15.reuse, R8, RZ ;
    # R10 = R15 + R8 + RZ

    # IADD3 R12, P2, R12, R13, RZ ;
    # R12 = R12 + mat_store_base_addr + RZ

    # IADD3 R14, P0, R15, R12, RZ ;
    # R14 = R15 + R12 + RZ

    res.append(mat0)
    res.append(mat1)
    res.append(mat2)
    res.append(mat3)
    res.append(mat4)
    res.append(mat5)
    res.append(mat6)
    res.append(mat7)

    return res


def compute_column_major_mat16x16_offset_f32(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    byte_size = 4

    # IMAD.SHL.U32 R7, R6.reuse, 0x2, RZ ;
    R7 = lane_id * 0x2 + RZ

    # LOP3.LUT R6, R6, 0xfffffffc, RZ, 0xc0, !PT ;
    R6 = lane_id & 0xFFFFFFFC

    # LOP3.LUT R7, R7, 0x6, RZ, 0xe2, !PT ;
    R7 = R7 & 0x6

    # @P1 IMAD.SHL.U32 R15, R4, 0x4, RZ ;
    R15 = stride * byte_size + RZ

    # IMAD R6, R7, R15, R6 ;
    R6 = R7 * R15 + R6

    # @P0 IMAD.SHL.U32 R0, R2, 0x4, RZ ;
    R0 = element * byte_size + RZ

    # IMAD R8, R15, 0x8, R6 ;
    R8 = R15 * 0x8 + R6

    # IADD3 R4, R6, 0x20, RZ ;
    R4 = R6 + 0x20 + RZ

    # IADD3 R13, P0, R0, c[0x0][0x30], RZ ;
    R13 = R0 + mat_store_addr + RZ

    # IADD3 R12, R8, 0x20, RZ ;
    R12 = R8 + 0x20 + RZ

    # IADD3 R0, P3, R6, R13.reuse, RZ ;
    R0 = R6 + R13 + RZ

    # IADD3 R4, P0, R4, R13.reuse, RZ ;
    R4 = R4 + R13 + RZ

    # IADD3 R8, P1, R8, R13.reuse, RZ ;
    R8 = R8 + R13 + RZ

    # IADD3 R2, P3, R15, R0, RZ ;
    R2 = R15 + R0 + RZ

    # IADD3 R12, P2, R12, R13, RZ ;
    R12 = R12 + R13 + RZ

    # IADD3 R6, P0, R15.reuse, R4, RZ ;
    R6 = R15 + R4 + RZ

    # IADD3 R10, P1, R15.reuse, R8, RZ ;
    R10 = R15 + R8 + RZ

    # IADD3 R14, P0, R15, R12, RZ ;
    R14 = R15 + R12 + RZ

    res.append(R0)
    res.append(R2)
    res.append(R4)
    res.append(R6)
    res.append(R8)
    res.append(R10)
    res.append(R12)
    res.append(R14)

    assert res == compute_column_major_mat16x16_offset_f32_simplified(
        stride, element, lane_id
    )

    return res


def compute_row_major_mat16x16_offset_f16(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    # c[0x0][0x50] = stride buffer
    R4 = stride
    # c[0x0][0x40] = element buffer
    R2 = element

    # S2R R8, SR_LANEID ;

    # SHF.R.U32.HI R7, RZ, 0x2, R8
    R1 = lane_id >> 0x2

    # LOP3.LUT R7, R7, 0x3, RZ, 0xc0, !PT ;
    R7 = lane_id & 0x3

    # IMAD.SHL.U32 R6, R4, 0x2, RZ ;
    R6 = stride * 2

    # IMAD R8, R1, R6, RZ ;
    R8 = R1 * R6 + RZ

    # IMAD R7, R7, 0x4, R8 ;
    R7 = R7 * 0x4 + R8

    # SHF.L.U32 R0, R2, 0x1, RZ ;
    R0 = element * 2

    # IADD3 R4, R7, 0x10, RZ ;
    R4 = R7 + 0x10 + RZ

    # IMAD R2, R6, 0x8, R7 ;
    R2 = R6 * 0x8 + R7

    # IADD3 R3, P0, R0, c[0x0][0x30], RZ ;
    # c[0x0][0x30] = mat store addr
    R3 = R0 + mat_store_addr + RZ

    # IADD3 R0, P0, R7, R3.reuse, RZ ;
    # mat[0..1] addr for store
    R0 = R7 + R3 + RZ

    # IADD3 R6, R2, 0x10, RZ ;
    R6 = R2 + 0x10 + RZ

    # IADD3 R4, P2, R4, R3.reuse, RZ ;
    # mat[4..6] addr for store
    R4 = R4 + R3 + RZ

    # IADD3 R2, P1, R2, R3.reuse, RZ ;
    # mat[2..3] addr for store
    R2 = R2 + R3 + RZ

    # IADD3 R6, P3, R6, R3, RZ ;
    # mat[6..8] addr for store
    R6 = R6 + R3 + RZ

    res.append(R0 + 0)
    res.append(R0 + 2)
    res.append(R2 + 0)
    res.append(R2 + 2)
    res.append(R4 + 0)
    res.append(R4 + 2)
    res.append(R6 + 0)
    res.append(R6 + 2)

    return res


def compute_row_major_mat16x16_offset_u32(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    # c[0x0][0x50] = stride buffer
    R2 = element
    # c[0x0][0x40] = element buffer
    R4 = stride

    # SHF.R.U32.HI R1, RZ, 0x2, R7
    R1 = lane_id >> 0x2

    # LOP3.LUT R7, R7, 0x3, RZ, 0xc0, !PT ;
    R7 = lane_id & 0x3

    # @P1 SHF.L.U32 R6, R4, 0x2, RZ ;
    R6 = R4 * 4

    # IMAD R8, R1, R6, RZ ;
    R8 = R1 * R6 + RZ

    # IMAD R7, R7, 0x8, R8 ;
    R7 = R7 * 0x8 + R8

    # IMAD.MOV.U32 R8, RZ, RZ, 0x1 ;
    R8 = 0x1

    # @P0 IMAD.SHL.U32 R0, R2, 0x4, RZ ;
    R0 = R2 * 0x4 + RZ

    # @P0 MOV R1, RZ ;
    R1 = RZ

    # IMAD R2, R6, 0x8, R7 ;
    R2 = R6 * 0x8 + R7

    # IADD3 R4, R7, 0x20, RZ ;
    R4 = R7 + 0x20 + RZ

    # IADD3 R3, P0, R0, c[0x0][0x30], RZ ;
    R3 = R0 + mat_store_addr + RZ

    # IADD3 R6, R2, 0x20, RZ ;
    R6 = R2 + 0x20 + RZ

    # IADD3 R0, P0, R7, R3.reuse, RZ ;
    R0 = R7 + R3 + RZ

    # IADD3 R4, P2, R4, R3.reuse, RZ ;
    R4 = R4 + R3 + RZ

    # IADD3 R2, P1, R2, R3.reuse, RZ ;
    R2 = R2 + R3 + RZ

    # IADD3 R6, P3, R6, R3, RZ ;
    R6 = R6 + R3 + RZ

    res.append(R0 + 0)
    res.append(R0 + 4)
    res.append(R2 + 0)
    res.append(R2 + 4)
    res.append(R4 + 0)
    res.append(R4 + 4)
    res.append(R6 + 0)
    res.append(R6 + 4)

    return res


def compute_row_major_mat16x8_offset_f32(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    # c[0x0][0x50] = stride buffer
    R4 = stride
    # c[0x0][0x40] = element buffer
    R2 = element

    # S2R R8, SR_LANEID ;

    # SHF.R.U32.HI R7, RZ, 0x2, R8
    R7 = lane_id >> 0x2

    # IMAD.SHL.U32 R6, R4, 0x4, RZ ;
    # XXX: DIFF here
    R6 = R4 * 4

    # XXX: DIFF here (value)
    R4 = 0x3F800000

    # LOP3.LUT R8, R8, 0x3, RZ, 0xc0, !PT ;
    R8 = lane_id & 0x3

    # IMAD R7, R7, R6, RZ ;
    R7 = R7 * R6 + RZ

    # XXX: DIFF here (element size?)
    # SHF.L.U32 R0, R2, 0x2, RZ ;
    R0 = R2 * 4

    # XXX: NEW here (???)
    # IMAD R7, R8, 0x8, R7 ;
    R7 = R8 * 0x8 + R7

    # LEA R2, R6, R7, 0x3 ;
    # XXX: DIFF here (register changed, shift is now 0x3)
    R2 = (R6 << 0x3) + R7

    # IADD3 R3, P0, R0, c[0x0][0x30], RZ ;
    # c[0x0][0x30] = mat store addr
    R3 = R0 + mat_store_addr + RZ

    # IMAD R2, R6, 0x8, R7 ;
    # XXX: REMOVED here
    # R2 = R6 * 0x8 + R7

    # IADD3 R0, P0, R7, R3.reuse, RZ ;
    # mat[0..1] addr for store
    R0 = R7 + R3 + RZ

    # IADD3 R2, P1, R2, R3, RZ ;
    # mat[2..3] addr for store
    R2 = R2 + R3 + RZ

    # print(f"mat[0..1] offset: 0x{R0:x}")
    # print(f"mat[2..3] offset: 0x{R2:x}")

    # STG.E.STRONG.GPU [R0], R5 ;
    # STG.E.STRONG.GPU [R2], R5 ;
    res.append(R0 + 0)
    res.append(R0 + 4)
    res.append(R2 + 0)
    res.append(R2 + 4)

    return res


def compute_row_major_mat16x8_offset_f16(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()
    # S2R R8, SR_LANEID ;

    # SHF.R.U32.HI R7, RZ, 0x2, R8
    R7 = lane_id >> 0x2

    # IMAD.SHL.U32 R6, R4, 0x2, RZ ;
    R6 = stride * 2

    # LOP3.LUT R8, R8, 0x3, RZ, 0xc0, !PT ;
    R8 = lane_id & 0x3

    # IMAD R7, R7, R6, RZ ;
    R7 = R7 * R6 + RZ

    # IMAD.SHL.U32 R0, R2, 0x2, RZ ;
    R0 = element * 2

    # LEA R7, R8, R7, 0x2 ;
    R7 = (R8 << 0x2) + R7

    # IADD3 R3, P0, R0, c[0x0][0x30], RZ ;
    # c[0x0][0x30] = mat store addr
    R3 = R0 + mat_store_addr + RZ

    # IMAD R2, R6, 0x8, R7 ;
    R2 = R6 * 0x8 + R7

    # IADD3 R0, P0, R7, R3.reuse, RZ ;
    # mat[0..1] addr for store
    R0 = R7 + R3 + RZ

    # IADD3 R2, P1, R2, R3, RZ ;
    # mat[2..3] addr for store
    R2 = R2 + R3 + RZ

    # STG.E.STRONG.GPU [R0], R5 ;
    # STG.E.STRONG.GPU [R2], R5 ;
    res.append(R0 + 0)
    res.append(R0 + 2)
    res.append(R2 + 0)
    res.append(R2 + 2)

    return res


def compute_column_major_mat16x8_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    R9 = lane_id

    # SHF.L.U32 R0, R9, 0x2, RZ ;
    R0 = R9 * 0x4

    # LOP3.LUT R0, R0, 0xc, RZ, 0xe2, !PT ;
    R0 = R0 & 0xC

    # IMAD R6, R0, R7, RZ ;
    R6 = R0 * stride

    # LEA.HI R9, R9, R6, RZ, 0x1e ;
    R9 = lea_hi(R9, R6, RZ, 0x1E)

    # IADD3 R0, P0, P1, R9, c[0x0][0x30], R0 ;
    R0 = R9 + mat_store_addr + element

    # IADD3 R2, P0, R7.reuse, R0, RZ ;
    R2 = stride + R0 + RZ

    # IADD3 R4, P0, R7, R2, RZ ;
    R4 = stride + R2 + RZ

    # IADD3 R6, P1, R7, R4, RZ ;
    R6 = stride + R4 + RZ

    res.append(R0)
    res.append(R2)
    res.append(R4)
    res.append(R6)

    return res


def compute_column_major_mat16x8_offset_f32(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    R6 = lane_id

    # IMAD.SHL.U32 R8, R6.reuse, 0x2, RZ ;
    R8 = lane_id * 0x2 + RZ

    # LOP3.LUT R6, R6, 0xfffffffc, RZ, 0xc0, !PT ;
    R6 = lane_id & 0xFFFFFFFC

    # LOP3.LUT R8, R8, 0x6, RZ, 0xe2, !PT ;
    R8 = R8 & 0x6

    # @P1 SHF.L.U32 R7, R4, 0x2, RZ ;
    R7 = stride * 0x4

    # @P0 IMAD.SHL.U32 R0, R2, 0x4, RZ ;
    R0 = element * 0x4 + RZ

    # IMAD R6, R8, R7, R6 ;
    R6 = R8 * R7 + R6

    # IADD3 R5, P0, R0, c[0x0][0x30], RZ ;
    R5 = R0 + mat_store_addr + RZ

    # IADD3 R4, R6.reuse, 0x20, RZ ;
    R4 = R6 + 0x20 + RZ

    # IADD3 R0, P0, R6, R5.reuse, RZ ;
    R0 = R6 + R5 + RZ

    # IADD3 R4, P1, R4, R5, RZ ;
    R4 = R4 + R5 + RZ

    # IADD3 R2, P0, R7.reuse, R0, RZ ;
    R2 = R7 + R0 + RZ

    # IADD3 R6, P1, R7, R4, RZ ;
    R6 = R7 + R4 + RZ

    res.append(R0)
    res.append(R2)
    res.append(R4)
    res.append(R6)

    return res


def compute_row_major_mat32x16_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()
    # c[0x0][0x50] = stride buffer
    R13 = stride

    # c[0x0][0x40] = element buffer
    R2 = element

    # S2R R7, SR_LANEID ;
    R7 = lane_id

    # @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
    R1 = RZ

    # IMAD.SHL.U32 R6, R7, 0x4, RZ ;
    R6 = R7 * 0x4 + RZ

    # LOP3.LUT R6, R6, 0xc, RZ, 0xe2, !PT ;
    # 0xe2 = (x & y) | (z & ~y)
    R6 = (R6 & 0xC) | (RZ & ~0xC)

    # IMAD R6, R6, R13, RZ ;
    R6 = R6 * R13 + RZ

    # @P0 IMAD.MOV.U32 R0, RZ, RZ, R2 ;
    R0 = R2

    # LEA.HI R6, R7, R6, RZ, 0x1e ;
    R6 = lea_hi(R7, R6, RZ, 0x1E)
    # R6 = (R7 >> 2) + R6

    # IMAD R7, R13, 0x10, R6 ;
    R7 = R13 * 0x10 + R6

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    # c[0x0][0x30] = mat store addr
    R8 = R0 + mat_store_addr + RZ

    # IMAD R7, R13, 0x10, R6 ;
    R7 = R13 * 0x10 + R6

    # IADD3 R2, R6.reuse, 0x8, RZ ;
    R2 = R6 + 0x8 + RZ

    # IADD3 R1, R7, 0x8, RZ ;
    R1 = R7 + 0x8 + RZ

    # IADD3 R0, P1, R6, R8.reuse, RZ ;
    R0 = R6 + R8 + RZ

    # IADD3 R6, P0, R1, R8.reuse, RZ ;
    R6 = R1 + R8 + RZ

    # IADD3 R2, P2, R2, R8.reuse, RZ ;
    R2 = R2 + R8 + RZ

    # IADD3 R4, P3, R7, R8, RZ ;
    R4 = R7 + R8 + RZ

    # IADD3 R8, P1, R13.reuse, R0, RZ ;
    R8 = R13 + R0 + RZ

    # STG.E.U8.STRONG.CTA [R0], R11 ;
    res.append(R0)

    # STG.E.U8.STRONG.CTA [R2], R12 ;
    res.append(R2)

    # STG.E.U8.STRONG.CTA [R4], R15 ;
    res.append(R4)

    # STG.E.U8.STRONG.CTA [R6], R16 ;
    res.append(R6)

    # STG.E.U8.STRONG.CTA [R8], R17 ;
    res.append(R8)

    # IADD3 R10, P1, R13, R2, RZ ;
    R10 = R13 + R2 + RZ

    # IADD3 R0, P0, R13.reuse, R4, RZ ;
    R0 = R13 + R4 + RZ

    # IADD3 R2, P0, R13, R6, RZ ;
    R2 = R13 + R6 + RZ

    # IADD3 R4, P1, R13.reuse, R8, RZ ;
    R4 = R13 + R8 + RZ

    # STG.E.U8.STRONG.CTA [R10], R18 ;
    res.append(R10)

    # IADD3 R6, P1, R13.reuse, R10, RZ ;
    R6 = R13 + R10 + RZ

    # IADD3 R8, P0, R13, R0, RZ ;
    R8 = R13 + R0 + RZ

    # STG.E.U8.STRONG.CTA [R0], R12 ;
    res.append(R0)

    # STG.E.U8.STRONG.CTA [R2], R15 ;
    res.append(R2)

    # STG.E.U8.STRONG.CTA [R4], R16 ;
    res.append(R4)

    # STG.E.U8.STRONG.CTA [R6], R17 ;
    res.append(R6)

    # STG.E.U8.STRONG.CTA [R8], R11 ;
    res.append(R8)

    # IADD3 R10, P2, R13.reuse, R6, RZ ;
    R10 = R13 + R6 + RZ
    # IADD3 R0, P0, R13.reuse, R2, RZ ;
    R0 = R13 + R2 + RZ
    # IADD3 R12, P1, R13.reuse, R8, RZ ;
    R12 = R13 + R8 + RZ
    # IADD3 R2, P0, R13, R4, RZ ;
    R2 = R13 + R4 + RZ
    # IADD3 R4, P3, R13, R0, RZ ;
    R4 = R13 + R0 + RZ

    # STG.E.U8.STRONG.CTA [R0], R18 ;
    res.append(R0)
    # STG.E.U8.STRONG.CTA [R2], R15 ;
    res.append(R2)
    # STG.E.U8.STRONG.CTA [R10], R16 ;
    res.append(R10)
    # STG.E.U8.STRONG.CTA [R12], R14 ;
    res.append(R12)
    # STG.E.U8.STRONG.CTA [R4], R6 ;
    res.append(R4)

    return res


def compute_column_major_mat32x16_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()
    # c[0x0][0x50] = stride buffer
    R6 = stride

    # c[0x0][0x40] = element buffer
    R2 = element

    # SHF.R.U32.HI R1, RZ, 0x2, R7 ;
    R1 = lane_id >> 0x2

    # LOP3.LUT R7, R7, 0x3, RZ, 0xc0, !PT ;
    R7 = lane_id & 0x3

    # IMAD R8, R1, R6, RZ ;
    R8 = R1 * R6 + RZ

    # IMAD R7, R7, 0x4, R8 ;
    R7 = R7 * 0x4 + R8

    # @P0 MOV R0, R2 ;
    R0 = R2

    # @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
    R1 = RZ

    # IADD3 R4, R7, 0x10, RZ ;
    R4 = R7 + 0x10 + RZ

    # IMAD R6, R6, 0x8, R7 ;
    R6 = R6 * 0x8 + R7

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    R8 = R0 + mat_store_addr + RZ

    # IADD3 R1, R6.reuse, 0x10, RZ ;
    R1 = R6 + 0x10 + RZ

    # IADD3 R0, P0, R7, R8.reuse, RZ ;
    R0 = R7 + R8 + RZ

    # IADD3 R2, P1, R6, R8.reuse, RZ ;
    R2 = R6 + R8 + RZ

    # IADD3 R6, P3, R1, R8.reuse, RZ ;
    R6 = R1 + R8 + RZ

    # IADD3 R4, P2, R4, R8, RZ ;
    R4 = R4 + R8 + RZ

    res.append(R0 + 0)
    res.append(R0 + 1)
    res.append(R0 + 2)
    res.append(R0 + 3)

    res.append(R2 + 0)
    res.append(R2 + 1)
    res.append(R2 + 2)
    res.append(R2 + 3)

    res.append(R4 + 0)
    res.append(R4 + 1)
    res.append(R4 + 2)
    res.append(R4 + 3)

    res.append(R6 + 0)
    res.append(R6 + 1)
    res.append(R6 + 2)
    res.append(R6 + 3)

    return res


def compute_row_major_mat16x32_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()
    # c[0x0][0x50] = stride buffer
    R6 = stride

    # c[0x0][0x40] = element buffer
    R2 = element

    # SHF.R.U32.HI R1, RZ, 0x2, R7 ;
    R1 = lane_id >> 0x2

    # LOP3.LUT R7, R7, 0x3, RZ, 0xc0, !PT ;
    R7 = lane_id & 0x3

    # IMAD R8, R1, R6, RZ ;
    R8 = R1 * R6 + RZ

    # IMAD R7, R7, 0x4, R8 ;
    R7 = R7 * 0x4 + R8

    # @P0 MOV R0, R2 ;
    R0 = R2

    # IMAD.MOV.U32 R1, RZ, RZ, RZ ;
    R1 = RZ

    # IADD3 R4, R7, 0x10, RZ ;
    R4 = R7 + 0x10

    # IMAD R6, R6, 0x8, R7 ;
    R6 = R6 * 0x8 + R7

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    # c[0x0][0x30] = mat store addr
    R8 = R0 + mat_store_addr + RZ

    # IADD3 R1, R6.reuse, 0x10, RZ ;
    R1 = R6 + 0x10 + RZ

    # IADD3 R0, P0, R7, R8.reuse, RZ ;
    R0 = R7 + R8 + RZ

    # IADD3 R2, P1, R6, R8.reuse, RZ ;
    R2 = R6 + R8 + RZ

    # IADD3 R6, P3, R1, R8.reuse, RZ ;
    R6 = R1 + R8 + RZ

    # IADD3 R4, P2, R4, R8, RZ ;
    R4 = R4 + R8 + RZ

    res.append(R0 + 0)
    res.append(R0 + 1)
    res.append(R0 + 2)
    res.append(R0 + 3)
    res.append(R2 + 0)
    res.append(R2 + 1)
    res.append(R2 + 2)
    res.append(R2 + 3)
    res.append(R4 + 0)
    res.append(R4 + 1)
    res.append(R4 + 2)
    res.append(R4 + 3)
    res.append(R6 + 0)
    res.append(R6 + 1)
    res.append(R6 + 2)
    res.append(R6 + 3)

    return res


def compute_column_major_mat16x32_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()
    # c[0x0][0x50] = stride buffer
    R13 = stride

    # c[0x0][0x40] = element buffer
    R2 = element

    # S2R R7, SR_LANEID ;
    R7 = lane_id

    # IMAD.SHL.U32 R6, R7, 0x4, RZ ;
    R6 = R7 * 0x4 + RZ

    # LOP3.LUT R6, R6, 0xc, RZ, 0xe2, !PT ;
    R6 = R6 & 0xC

    # IMAD R6, R6, R13, RZ ;
    R6 = R6 * R13 + RZ

    # IMAD.MOV.U32 R0, RZ, RZ, R2 ;
    R0 = R2

    # LEA.HI R6, R7, R6, RZ, 0x1e ;
    R6 = lea_hi(R7, R6, RZ, 0x1E)

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    R8 = R0 + mat_store_addr + RZ

    # IMAD R7, R13, 0x10, R6 ;
    R7 = R13 * 0x10 + R6

    # IADD3 R2, R6.reuse, 0x8, RZ ;
    R2 = R6 + 0x8 + RZ

    # IADD3 R1, R7, 0x8, RZ ;
    R1 = R7 + 0x8 + RZ

    # IADD3 R0, P1, R6, R8.reuse, RZ ;
    R0 = R6 + R8 + RZ

    # IADD3 R6, P0, R1, R8.reuse, RZ ;
    R6 = R1 + R8 + RZ

    # IADD3 R2, P2, R2, R8.reuse, RZ ;
    R2 = R2 + R8 + RZ

    # IADD3 R4, P3, R7, R8, RZ ;
    R4 = R7 + R8 + RZ

    # IADD3 R8, P1, R13.reuse, R0, RZ ;
    R8 = R13 + R0 + RZ

    res.append(R0)
    res.append(R2)

    # IADD3 R10, P1, R13, R2, RZ ;
    R10 = R13 + R2 + RZ

    res.append(R4)

    # IADD3 R0, P0, R13.reuse, R4, RZ ;
    R0 = R13 + R4 + RZ

    res.append(R6)

    # IADD3 R2, P0, R13, R6, RZ ;
    R2 = R13 + R6 + RZ

    res.append(R8)

    # IADD3 R4, P1, R13.reuse, R8, RZ ;
    R4 = R13 + R8 + RZ

    res.append(R10)

    # IADD3 R6, P1, R13.reuse, R10, RZ ;
    R6 = R13 + R10 + RZ

    res.append(R0)

    # IADD3 R8, P0, R13, R0, RZ ;
    R8 = R13 + R0 + RZ

    res.append(R2)

    # IADD3 R10, P2, R13.reuse, R6, RZ ;
    R10 = R13 + R6 + RZ

    res.append(R4)

    # IADD3 R0, P0, R13.reuse, R2, RZ ;
    R0 = R13 + R2 + RZ

    # IADD3 R12, P1, R13.reuse, R8, RZ ;
    R12 = R13 + R8 + RZ

    res.append(R6)

    # IADD3 R2, P0, R13, R4, RZ ;
    R2 = R13 + R4 + RZ

    res.append(R8)

    # IADD3 R4, P3, R13, R0, RZ ;
    R4 = R13 + R0 + RZ

    res.append(R0)
    res.append(R2)
    res.append(R10)
    res.append(R12)
    res.append(R4)

    return res


def compute_row_major_mat16x8_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    # c[0x0][0x50] = stride buffer
    R6 = stride

    # c[0x0][0x40] = element buffer
    R2 = element

    R9 = lane_id

    # SHF.R.U32.HI R7, RZ, 0x2, R9 ;
    R7 = R9 >> 0x2

    # IMAD R8, R7, R6, RZ ;
    R8 = R7 * R6 + RZ

    # LOP3.LUT R7, R9, 0x3, RZ, 0xc0, !PT ;
    R7 = R9 & 0x3

    # @P0 IMAD.MOV.U32 R0, RZ, RZ, R2 ;
    R0 = R2

    # IMAD R7, R7, 0x4, R8 ;
    R7 = R7 * 0x4 + R8

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    # c[0x0][0x30] = mat store addr
    R8 = R0 + mat_store_addr + RZ

    # IMAD R6, R6, 0x8, R7 ;
    R6 = R6 * 0x8 + R7

    # IADD3 R4, R7, 0x10, RZ ;
    R4 = R7 + 0x10 + RZ

    # IADD3 R1, R6.reuse, 0x10, RZ ;
    R1 = R6 + 0x10 + RZ

    # IADD3 R0, P0, R7, R8.reuse, RZ ;
    R0 = R7 + R8 + RZ

    # IADD3 R2, P1, R6, R8.reuse, RZ ;
    R2 = R6 + R8 + RZ

    # IADD3 R4, P2, R4, R8, RZ ;
    R4 = R4 + R8 + RZ

    # IADD3 R6, P3, R1, R8, RZ ;
    R6 = R1 + R8 + RZ

    res.append(R0)
    res.append(R2)
    res.append(R4)
    res.append(R6)

    return res


def compute_row_major_mat32x8_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    # c[0x0][0x40] = stride buffer
    R11 = stride

    # c[0x0][0x50] = element buffer
    R2 = element

    R7 = lane_id

    # IMAD.SHL.U32 R0, R7, 0x4, RZ ;
    R0 = R7 * 0x4

    # LOP3.LUT R0, R0, 0xc, RZ, 0xe2, !PT ;
    R0 = (R0 & 0xC) | (RZ & ~0xC)

    # IMAD R6, R0, R11, RZ ;
    R6 = R0 * R11 + RZ

    # LEA.HI R6, R7, R6, RZ, 0x1e ;
    R6 = lea_hi(R7, R6, RZ, 0x1E)

    # @P0 IMAD.MOV.U32 R0, RZ, RZ, R2 ;
    R0 = R2

    # @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
    R1 = RZ

    # IMAD R7, R11, 0x10, R6 ;
    R7 = R11 * 0x10 + R6

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    # c[0x0][0x30] = mat store addr
    R8 = R0 + mat_store_addr + RZ

    # IADD3 R0, P1, R6, R8.reuse, RZ ;
    R0 = R6 + R8 + RZ

    # IADD3 R2, P0, R7, R8, RZ ;
    R2 = R7 + R8 + RZ

    # IADD3 R4, P1, R11.reuse, R0, RZ ;
    R4 = R11 + R0 + RZ

    # IADD3 R6, P0, R11, R2, RZ ;
    R6 = R11 + R2 + RZ

    res.append(R0)
    res.append(R2)
    res.append(R4)
    res.append(R6)

    # IADD3 R8, P1, R11.reuse, R4, RZ ;
    R8 = R11 + R4 + RZ

    # IADD3 R0, P0, R11, R6, RZ ;
    R0 = R11 + R6 + RZ

    # IADD3 R10, P1, R11, R8, RZ ;
    R10 = R11 + R8 + RZ

    # IADD3 R2, P0, R11, R0, RZ ;
    R2 = R11 + R0 + RZ

    res.append(R8)
    res.append(R0)
    res.append(R10)
    res.append(R2)

    return res


def compute_column_major_mat32x8_offset_u8(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    # c[0x0][0x40] = stride buffer
    R6 = stride

    # c[0x0][0x50] = element buffer
    R2 = element

    # S2R R8, SR_LANEID ;
    R8 = lane_id

    # IMAD.MOV.U32 R1, RZ, RZ, RZ ;
    R1 = RZ

    # SHF.R.U32.HI R7, RZ, 0x2, R8 ;
    R7 = R8 >> 0x2

    # IMAD R7, R7, R6, RZ ;
    R7 = R7 * R6 + RZ

    # LOP3.LUT R6, R8, 0x3, RZ, 0xc0, !PT ;
    R6 = R8 & 0x3

    # @P0 IMAD.MOV.U32 R0, RZ, RZ, R2 ;
    R0 = R2

    # LEA R6, R6, R7, 0x2 ;
    R6 = (R6 << 0x2) + R7

    # IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
    R8 = R0 + mat_store_addr + RZ

    # IADD3 R7, R6.reuse, 0x10, RZ ;
    R7 = R6 + 0x10 + RZ

    # IADD3 R0, P0, R6, R8.reuse, RZ ;
    R0 = R6 + R8 + RZ

    # IADD3 R2, P1, R7, R8, RZ ;
    R2 = R7 + R8 + RZ

    res.append(R0 + 0)
    res.append(R0 + 1)
    res.append(R0 + 2)
    res.append(R0 + 3)

    res.append(R2 + 0)
    res.append(R2 + 1)
    res.append(R2 + 2)
    res.append(R2 + 3)

    return res


def compute_row_major_mat8x8_offset_f16(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    R2 = element
    R4 = stride

    # SHF.R.U32.HI R1, RZ, 0x2, R6 ;
    R1 = lane_id >> 0x2

    # LOP3.LUT R6, R6, 0x3, RZ, 0xc0, !PT ;
    R6 = lane_id & 0x3

    # @P1 SHF.L.U32 R0, R4, 0x1, RZ ;
    R0 = R4 * 2

    # IMAD R7, R1, R0, RZ ;
    R7 = R1 * R0 + RZ

    # LEA R7, R6, R7, 0x2 ;
    R7 = (R6 << 0x2) + R7

    # IMAD.SHL.U32 R0, R2, 0x2, RZ ;
    R0 = R2 * 0x2 + RZ

    # IMAD.MOV.U32 R1, RZ, RZ, RZ ;
    R1 = RZ

    # IADD3 R0, P0, P1, R7, c[0x0][0x30], R0 ;
    R0 = R7 + mat_store_addr + R0

    # STG.E.STRONG.CTA [R0], R3 ;
    res.append(R0)
    res.append(R0 + 2)

    return res


def exec_shader(
    stride: int,
    element: int,
    lane_id: int,
    user_data: Tuple[Path, int],
) -> List[int]:
    (shader_path, expected_element_size) = user_data
    res = list()

    # print(f"loading shader {shader_path}")
    info = parse_assembly_from_file(shader_path)

    bindings = list()

    # Output buffer (set=0, binding=0)
    bindings.append([0] * 0x8000)

    # Stride buffer (set=0, binding=1)
    bindings.append([stride])

    # Unused buffer (set=0, binding=2)
    # bindings.append([])

    # Element buffer (set=0, binding=3)
    bindings.append([element])

    def read_cbuf_value(
        cbuf_idx: int, cbuf_offset: int, extra_offset: int, element_size: int
    ) -> int:
        assert cbuf_idx == 0
        assert element_size == 32

        cbuf_offset += extra_offset

        # set/binding mappings (XXX: max offset is unk, we assume 8 max for now)
        if cbuf_offset >= 0x30 and cbuf_offset <= 0xB0:
            binding_idx = (cbuf_offset - 0x30) // 0x10
            is_size = ((cbuf_offset - 0x30) % 0x10) == 0x8

            # We are assuming 1MiB max of data per bindings
            # And process fake address accordingly.
            max_buffer_size = 0x100000

            if not is_size:
                return max_buffer_size * (binding_idx + 1)

            if len(bindings) <= binding_idx:
                print(f"out of bound {binding_idx}")
                return 0

            return len(bindings[binding_idx]) * 0x4

        print(f"Unknown cbuf offset 0x{cbuf_offset:x}")

        return 0

    def read_special_reg_value(special_reg_name: str) -> int:
        if special_reg_name == "SR_LANEID":
            return lane_id

        print(special_reg_name)
        assert special_reg_name == "SR_LANEID"

        return 0

    def read_global_value(address: int, element_size: int) -> int:
        assert element_size <= 32

        element_mask = (1 << element_size) - 1

        value = 0

        if address >= 0x100000 and address < 0x800000:
            binding_idx = (address // 0x100000) - 1
            offset = address % 0x100000

            aligned_offset = offset // 4
            value_shift = (offset % 4) * 8

            value = bindings[binding_idx][aligned_offset]
            value = value >> value_shift
        else:
            print(f"Unknown global read at address 0x{address:x}")

        return value & element_mask

    def is_float_value_valid(val: int, val_size: int) -> bool:
        if val_size == 4:
            return val == 0x3F800000 or val == 0x00000001
        elif val_size == 2:
            return val == 0x3C003C00 or val == 0x3C00
        elif val_size == 1:
            return val == 0x1010101 or val == 0x0101 or val == 0x01

        raise Exception(f"Unknown val_size {val_size}")

    def write_global_value(address: int, value: int, element_size: int):
        byte_element_size = element_size // 8
        base_offset = address % 0x100000

        if not is_float_value_valid(value, expected_element_size):
            print(
                "WARN: Impossible values reported on write! Is the interpreter drunk?"
            )
            print((hex(value), expected_element_size))
            print(shader_path)

        for i in range(byte_element_size // expected_element_size):
            offset = base_offset + i * expected_element_size
            res.append(offset)

        # print(f"write_global_value: 0x{address:x} = 0x{value:x} (size: {element_size})")
        pass

    ctx = EmulatorContext(
        info,
        read_cbuf_value,
        read_special_reg_value,
        read_global_value,
        write_global_value,
    )
    ctx.run(0, False)

    return res


def compute_row_major_mat8x8_offset_f16_emulated(
    stride: int, element: int, lane_id: int, user_data
) -> List[int]:
    return exec_shader(stride, element, lane_id, user_data)


def compute_row_major_mat8x8_offset_u32(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    R4 = stride

    # SHF.R.U32.HI R1, RZ, 0x2, R6 ;
    R1 = lane_id >> 0x2

    # LOP3.LUT R6, R6, 0x3, RZ, 0xc0, !PT ;
    R6 = lane_id & 0x3

    # @P1 IMAD.SHL.U32 R0, R4, 0x4, RZ ;
    R0 = R4 * 4

    # IMAD R7, R1, R0, RZ ;
    R7 = R1 * R0 + RZ

    # IMAD R7, R6, 0x8, R7 ;
    R7 = R6 * 0x8 + R7

    # @P0 SHF.L.U32 R0, R2, 0x2, RZ ;
    R0 = element * 4

    # IADD3 R0, P0, P1, R7, c[0x0][0x30], R0 ;
    R0 = R7 + mat_store_addr + R0

    # STG.E.64.STRONG.CTA [R0], R2 ;
    res.append(R0)
    res.append(R0 + 4)

    return res


def compute_column_major_mat8x8_offset_u32(
    stride: int, element: int, lane_id: int
) -> List[int]:
    res = list()

    R2 = element
    R4 = stride

    # IMAD.SHL.U32 R8, R6.reuse, 0x2, RZ ;
    R8 = lane_id * 0x2 + RZ

    # LOP3.LUT R6, R6, 0xfffffffc, RZ, 0xc0, !PT ;
    R6 = lane_id & 0xFFFFFFFC

    # LOP3.LUT R8, R8, 0x6, RZ, 0xe2, !PT ;
    R8 = R8 & 0x6

    # @P1 SHF.L.U32 R7, R4, 0x2, RZ ;
    R7 = R4 * 4

    # IMAD R9, R8, R7, R6 ;
    R9 = R8 * R7 + R6

    # @P0 IMAD.SHL.U32 R0, R2, 0x4, RZ ;
    R0 = R2 * 0x4 + RZ

    # IADD3 R0, P0, P1, R9, c[0x0][0x30], R0 ;
    R0 = R9 + mat_store_addr + R0

    # IADD3 R2, P0, R7, R0, RZ ;
    R2 = R7 + R0 + RZ

    res.append(R0)
    res.append(R2)

    return res


VERBOSE_LOG = True


def test_variant(
    row: int,
    column: int,
    stride: int,
    element: int,
    byte_size: int,
    is_col_major: bool,
    user_data: object,
    orig_func,
) -> bool:
    success = True

    if is_col_major:
        print(f"{column}x{row} (Column Major, {byte_size}b per element)")
    else:
        print(f"{row}x{column} (Row Major, {byte_size}b per element)")

    expected_mat_val_count = (column * row) // 32

    for lane_id in range(32):
        if not user_data:
            mat_orig_offsets = orig_func(stride, element, lane_id)
        else:
            mat_orig_offsets = orig_func(stride, element, lane_id, user_data)

        if len(mat_orig_offsets) * 32 > column * row:
            success = None
            print(
                "WARN: NVIDIA blobs is generating more values than possible per thread"
            )
            break

        assert expected_mat_val_count == len(mat_orig_offsets)

        mat_computed_offsets = list()

        for i in range(expected_mat_val_count):
            value = compute_mat_offset_new(
                row,
                col,
                stride,
                element,
                byte_size,
                lane_id,
                i,
                is_col_major,
            )
            mat_computed_offsets.append(value)

        assert len(mat_orig_offsets) == len(mat_computed_offsets)

        if mat_orig_offsets != mat_computed_offsets:
            if VERBOSE_LOG:
                print(f"Mismatch at LANE_ID = {lane_id}")
                print(f"    orig = {mat_orig_offsets}")
                print(f"    us   = {mat_computed_offsets}")
                print("============\n")
            success = False
            break

    if success is None:
        print("Result = Broken")
    elif success:
        print("Result = Success")
    else:
        print("Result = Fail")
    print("========")


selected_element = 69
selected_stride = 123

selected_element = 0
selected_stride = 1

# 16x8
TEST_CASES_16x8 = [
    # U8/S8 (Row Major) (FIXME: INVALID???)
    # (16, 8, 1, False, None, compute_row_major_mat16x8_offset_u8),
    # F16 (Row Major)
    (16, 8, 2, False, None, compute_row_major_mat16x8_offset_f16),
    # F32 (Row Major)
    (16, 8, 4, False, None, compute_row_major_mat16x8_offset_f32),
    # U8/S8 (Column Major) (FIXME: INVALID???)
    # (8, 16, 1, True, None, compute_column_major_mat16x8_offset_u8),
    # F16 (Column Major)
    # TODO: NVIDIA cheat and use MOVM here for F16, we can use F32 to get the layout.
    # F32 (Column Major)
    (8, 16, 4, True, None, compute_column_major_mat16x8_offset_f32),
]

# 16x16
TEST_CASES_16x16 = [
    # U8/S8 (Row Major) (FIXME: INVALID???)
    # (16, 16, 1, False, None, compute_row_major_mat16x16_offset_u8),
    # F16 (Row Major)
    (16, 16, 2, False, None, compute_row_major_mat16x16_offset_f16),
    # F32/U32 (Row Major)
    (16, 16, 4, False, None, compute_row_major_mat16x16_offset_u32),
    # U8/S8 (Column Major) (FIXME: INVALID???)
    # (16, 16, 1, True, None, compute_column_major_mat16x16_offset_u8),
    # F16 (Column Major)
    # TODO: NVIDIA cheat and use MOVM here for F16, we can use F32 to get the layout.
    # F32/U32/S32 (Column Major)
    (16, 16, 4, True, None, compute_column_major_mat16x16_offset_f32),
]

# (row, col, byte_size, is_col_major, user_data, orig_func)
TEST_CASES = (
    TEST_CASES_16x8
    + TEST_CASES_16x16
    + [
        # 32x16 U8/S8 (Row Major)
        (32, 16, 1, False, None, compute_row_major_mat32x16_offset_u8),
        # 16x32 U8/S8 (Row Major)
        (16, 32, 1, False, None, compute_row_major_mat16x32_offset_u8),
        # 32x8 U8/S8 (Row Major)
        (32, 8, 1, False, None, compute_row_major_mat32x8_offset_u8),
        # 32x16 U8/S8 (Column Major)
        (16, 32, 1, True, None, compute_column_major_mat32x16_offset_u8),
        # 16x32 U8/S8 (Column Major)
        (32, 16, 1, True, None, compute_column_major_mat16x32_offset_u8),
        # 32x8 U8/S8 (Column Major)
        (8, 32, 1, True, None, compute_column_major_mat32x8_offset_u8),
        # 8x8 F16 (Row Major)
        (8, 8, 2, False, None, compute_row_major_mat8x8_offset_f16),
        # 8x8 U32 (Row Major)
        (8, 8, 4, False, None, compute_row_major_mat8x8_offset_u32),
        # 8x8 U32 (Column Major)
        (8, 8, 4, True, None, compute_column_major_mat8x8_offset_u32),
    ]
)


TEST_CASES = []


def add_shader_test(
    output_directory: Path,
    vk_type: str,
    row: int,
    col: int,
    usage: str,
    layout_name: str,
):
    byte_size = VK_TYPE_TO_BYTE_SIZE[vk_type]
    short_usage = MATRIX_USAGE_SHORT_NAME[usage]
    shader_name = f"matrix_{vk_type.lower()}_{short_usage}_{row}x{col}.asm"
    full_path = output_directory.joinpath(shader_name)

    is_col_major = layout_name == "column"

    case_lambda = lambda stride, element, lane_id, user_data: exec_shader(
        stride, element, lane_id, user_data
    )

    info = parse_assembly_from_file(full_path)

    for instr in info.values():
        if "MOVM" in instr:
            print((col, row, is_col_major))
            print("MOVM in use, ignore transposing as we don't support it right now...")
            is_col_major = not is_col_major

    # (row, col, byte_size, is_col_major, user_data, orig_func)
    TEST_CASES.append(
        (row, col, byte_size, is_col_major, (full_path, byte_size), case_lambda)
    )


def append_shader_tests(output_directory: Path, entry: Dict[str, object]):
    a_type = entry["a_type"]
    b_type = entry["b_type"]
    c_type = entry["c_type"]
    result_type = entry["result_type"]
    m_size = entry["m_size"]
    n_size = entry["n_size"]
    k_size = entry["k_size"]
    matrix_name = f"{m_size}x{n_size}x{k_size}"

    for layout_name, store_layout in [
        ("row", "gl_CooperativeMatrixLayoutRowMajor"),
        ("column", "gl_CooperativeMatrixLayoutColumnMajor"),
    ]:
        final_output_directory = output_directory.joinpath(matrix_name).joinpath(
            layout_name
        )

        # MxNxK (A/B/C/D)
        # MxK (A)
        add_shader_test(
            final_output_directory,
            a_type,
            m_size,
            k_size,
            "gl_MatrixUseA",
            layout_name,
        )
        # KxN (B)
        add_shader_test(
            final_output_directory,
            b_type,
            k_size,
            n_size,
            "gl_MatrixUseB",
            layout_name,
        )
        # MxN (C)
        add_shader_test(
            final_output_directory,
            c_type,
            m_size,
            n_size,
            "gl_MatrixUseAccumulator",
            layout_name,
        )

        # MxN (D)
        if c_type != result_type:
            add_shader_test(
                final_output_directory,
                result_type,
                m_size,
                n_size,
                "gl_MatrixUseAccumulator",
                layout_name,
            )


# XXX: Need work
# for entry in SUPPORTED_CFGS_SM75:
#    append_shader_tests(Path("./coop_matrix_layout_store_shaders/sm75"), entry)

# for entry in SUPPORTED_CFGS_SM86:
#    append_shader_tests(Path("./coop_matrix_layout_store_shaders/sm86"), entry)

SUPPORTED_CFG_DEBUG = [
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT32",
        "result_type": "FLOAT32",
        "saturating_accumulation": 0,
    },
]

for entry in SUPPORTED_CFG_DEBUG:
    append_shader_tests(Path("./coop_matrix_layout_store_shaders/sm86"), entry)

print(f"selected_element = {selected_element}")
print(f"selected_stride = {selected_stride}")
print("========")

for test_case in TEST_CASES:
    (row, col, byte_size, is_col_major, user_data, orig_func) = test_case

    test_variant(
        row,
        col,
        selected_stride,
        selected_element,
        byte_size,
        is_col_major,
        user_data,
        orig_func,
    )


sys.exit(0)
