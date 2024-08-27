import sys
from typing import List


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
    is_colmn_major_16x32_u8 = (
        byte_size == 1 and is_colmn_major and row == 32 and column == 16
    )
    is_colmn_major_32x8_u8 = (
        byte_size == 1 and is_colmn_major and row == 8 and column == 32
    )

    idx_a = hw_idx // load_per_matrix_per_thread
    idx_b = hw_idx % load_per_matrix_per_thread

    if is_row_major_32x8_u8:
        idx_a = hw_idx // 2
        idx_b = hw_idx % 2

    if is_row_major_32x8_u8 or is_row_major_32x16_u8 or is_colmn_major_16x32_u8:
        tmp = idx_a
        idx_a = idx_b
        idx_b = tmp

    minor_offset += idx_b

    if is_row_major_32x8_u8 or is_colmn_major_32x8_u8:
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
    if (
        (is_colmn_major and not (col == 32 and (row == 8 or row == 16)))
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
    orig_func,
) -> bool:
    success = True

    if is_col_major:
        print(f"{column}x{row} (Column Major, {byte_size}b per element)")
    else:
        print(f"{row}x{column} (Row Major, {byte_size}b per element)")

    expected_mat_val_count = (column * row) // 32

    for lane_id in range(32):
        mat_orig_offsets = orig_func(stride, element, lane_id)

        assert expected_mat_val_count == len(mat_orig_offsets)

        mat_computed_offsets = list()

        for i in range(expected_mat_val_count):
            value = compute_mat_offset(
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
            continue

    if success:
        print("Result = Success")
    else:
        print("Result = Fail")
    print("========")


selected_element = 69
selected_stride = 123

# selected_element = 0
# selected_stride = 512

# 16x8
TEST_CASES_16x8 = [
    # U8/S8 (Row Major) (FIXME: INVALID???)
    # (16, 8, 1, False, compute_row_major_mat16x8_offset_u8),
    # F16 (Row Major)
    (16, 8, 2, False, compute_row_major_mat16x8_offset_f16),
    # F32 (Row Major)
    (16, 8, 4, False, compute_row_major_mat16x8_offset_f32),
    # U8/S8 (Column Major) (FIXME: INVALID???)
    # (8, 16, 1, True, compute_column_major_mat16x8_offset_u8),
    # F16 (Column Major)
    # TODO: NVIDIA cheat and use MOVM here for F16, we can use F32 to get the layout.
    # F32 (Column Major)
    (8, 16, 4, True, compute_column_major_mat16x8_offset_f32),
]

# 16x16
TEST_CASES_16x16 = [
    # U8/S8 (Row Major) (FIXME: INVALID???)
    # (16, 16, 1, False, compute_row_major_mat16x16_offset_u8),
    # F16 (Row Major)
    (16, 16, 2, False, compute_row_major_mat16x16_offset_f16),
    # F32/U32 (Row Major)
    (16, 16, 4, False, compute_row_major_mat16x16_offset_u32),
    # U8/S8 (Column Major) (FIXME: INVALID???)
    # (16, 16, 1, True, compute_column_major_mat16x16_offset_u8),
    # F16 (Column Major)
    # TODO: NVIDIA cheat and use MOVM here for F16, we can use F32 to get the layout.
    # F32/U32/S32 (Column Major)
    (16, 16, 4, True, compute_column_major_mat16x16_offset_f32),
]

# (row, col, byte_size, is_col_major, orig_func)
TEST_CASES = (
    TEST_CASES_16x8
    + TEST_CASES_16x16
    + [
        # 32x16 U8/S8 (Row Major)
        (32, 16, 1, False, compute_row_major_mat32x16_offset_u8),
        # 16x32 U8/S8 (Row Major)
        (16, 32, 1, False, compute_row_major_mat16x32_offset_u8),
        # 32x8 U8/S8 (Row Major)
        (32, 8, 1, False, compute_row_major_mat32x8_offset_u8),
        # 32x16 U8/S8 (Column Major)
        (16, 32, 1, True, compute_column_major_mat32x16_offset_u8),
        # 16x32 U8/S8 (Column Major)
        (32, 16, 1, True, compute_column_major_mat16x32_offset_u8),
        # 32x8 U8/S8 (Column Major)
        (8, 32, 1, True, compute_column_major_mat32x8_offset_u8),
        # 8x8 F16 (Row Major)
        (8, 8, 2, False, compute_row_major_mat8x8_offset_f16),
        # 8x8 U32 (Row Major)
        (8, 8, 4, False, compute_row_major_mat8x8_offset_u32),
        # 8x8 U32 (Column Major)
        (8, 8, 4, True, compute_column_major_mat8x8_offset_u32),
    ]
)


# TEST_CASES = [
#    (32, 8, 1, False, compute_row_major_mat32x8_offset_u8),
#    (8, 32, 1, True, compute_column_major_mat32x8_offset_u8),
# ]

print(f"selected_element = {selected_element}")
print(f"selected_stride = {selected_stride}")
print("========")

for test_case in TEST_CASES:
    (row, col, byte_size, is_col_major, orig_func) = test_case

    test_variant(
        row,
        col,
        selected_stride,
        selected_element,
        byte_size,
        is_col_major,
        orig_func,
    )


sys.exit(0)
