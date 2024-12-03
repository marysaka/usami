import sys
from typing import List

from matrix_shader_runner import *
from coop_matrix_defs import (
    MATRIX_USAGE_SHORT_NAME,
    SUPPORTED_CFGS_SM75,
    SUPPORTED_CFGS_SM86,
    VK_TYPE_TO_BYTE_SIZE,
)


RZ = 0
mat_store_addr = 0

LOAD_PER_MATRIX_PER_THREAD = 2
LOAD_PER_MATRIX_PER_THREAD_U8 = 4

HW_MATRIX_88 = 8 * 8
THREAD_COUNT = 32


# 16x8x8 and 16x8x16 follow the same layout mostly
def compute_16x8x16_target_by_lane_id(
    lane_id: int, idx: int, short_usage: str
) -> Tuple[int, int]:
    group_id = lane_id >> 2
    thread_id_in_group = lane_id % 4
    row = 0
    col = 0

    if short_usage == "use_a" or short_usage == "use_c":
        if idx == 0 or idx == 1:
            row = group_id
        elif idx == 2 or idx == 3:
            row = group_id + 8
        col = thread_id_in_group * 2 + (idx & 1)
    elif short_usage == "use_b":
        row = thread_id_in_group * 2 + (idx & 1)
        if idx >= 2:
            row += 8

        col = group_id
    else:
        raise Exception("BROKEN")

    return (row, col)


def compute_16x8x32_target_by_lane_id(
    lane_id: int, idx: int, short_usage: str, is_colmn_major: bool
) -> Tuple[int, int]:
    group_id = lane_id >> 2
    thread_id_in_group = lane_id % 4
    row = 0
    col = 0

    if short_usage == "use_a":
        if (idx >= 0 and idx < 4) or (idx >= 8 and idx < 12):
            row = group_id
        else:
            row = group_id + 8

        col = thread_id_in_group * 4 + (idx & 0x3)

        if idx >= 8:
            col += 16
    elif short_usage == "use_b":
        col = group_id
        row = thread_id_in_group * 4 + (idx & 0x3)
        if idx >= 4:
            row += 16
    else:
        raise Exception("BROKEN")

    print((row, col))
    return (row, col)


def compute_mat_offset_new(
    stride: int,
    element: int,
    vk_type: str,
    lane_id: int,
    hw_idx: int,
    is_colmn_major: bool,
    short_usage: str,
    matrix_layout_name: str,
) -> int:
    byte_size = VK_TYPE_TO_BYTE_SIZE[vk_type]
    mat_store_base_addr = mat_store_addr + element * byte_size

    target_row = 0
    target_col = 0

    if matrix_layout_name in ["16x8x8", "16x8x16", "16x16x16"] and vk_type in [
        "FLOAT16",
        "FLOAT32",
    ]:
        (target_row, target_col) = compute_16x8x16_target_by_lane_id(
            lane_id, hw_idx % 4, short_usage
        )
        target_col += (hw_idx // 4) * 8
    elif matrix_layout_name in ["16x8x32"] and vk_type in ["UINT8", "SINT8"]:
        (target_row, target_col) = compute_16x8x32_target_by_lane_id(
            lane_id, hw_idx % 16, short_usage, is_colmn_major
        )
        # XXX: Matrix bigger than 16x8x32
        # target_col += (hw_idx // 8) * 16
    else:
        raise Exception(f"Unknown matrix layout {matrix_layout_name} with {vk_type}")

    if is_colmn_major:
        major_offset = target_col * stride
        minor_offset = target_row
    else:
        major_offset = target_row * stride
        minor_offset = target_col

    offset = (major_offset + minor_offset) * byte_size

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


def exec_shader(
    stride: int,
    element: int,
    lane_id: int,
    user_data: Tuple[Path, int],
) -> List[int]:
    (shader_path, expected_mat_val_count, expected_element_size) = user_data
    #print(f"Running shader {shader_path}")

    res = dict()

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

        val_count = byte_element_size // expected_element_size

        for i in range(val_count):
            element_offset_byte = i * expected_element_size
            element_val_shift = element_offset_byte * 8
            element_val_mask = (1 << (expected_element_size * 8)) - 1

            extracted_val = (value >> element_val_shift) & element_val_mask

            offset = base_offset + element_offset_byte
            res[extracted_val] = offset

        #print(
        #    f"write_global_value: 0x{address:x} = 0x{value:x} (size: {element_size}, {expected_element_size})"
        #)
        pass

    ctx = EmulatorContext(
        info,
        read_cbuf_value,
        read_special_reg_value,
        read_global_value,
        write_global_value,
    )
    ctx.run(0, False)

    rearanged_res = list()

    for i in range(1, 64):
        val = res.get(i)
        if not val:
            break

        rearanged_res.append(val)

    return rearanged_res


def compute_row_major_mat8x8_offset_f16_emulated(
    stride: int, element: int, lane_id: int, user_data
) -> List[int]:
    return exec_shader(stride, element, lane_id, user_data)


VERBOSE_LOG = True


def test_variant(
    row: int,
    column: int,
    stride: int,
    element: int,
    vk_type: int,
    is_col_major: bool,
    short_usage: str,
    matrix_layout_name: str,
    user_data: object,
    orig_func,
) -> bool:
    success = True

    if is_col_major:
        print(
            f"{column}x{row} (Column Major, {short_usage}, {vk_type}, {matrix_layout_name})"
        )
    else:
        print(
            f"{row}x{column} (Row Major, {short_usage}, {vk_type}, {matrix_layout_name})"
        )

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
                stride,
                element,
                vk_type,
                lane_id,
                i,
                is_col_major,
                short_usage,
                matrix_layout_name,
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

selected_element = 50
selected_stride = 200

TEST_CASES = []


def add_shader_test(
    output_directory: Path,
    vk_type: str,
    row: int,
    col: int,
    usage: str,
    layout_name: str,
    matrix_layout_name: str,
):
    byte_size = VK_TYPE_TO_BYTE_SIZE[vk_type]
    short_usage = MATRIX_USAGE_SHORT_NAME[usage]
    shader_name = f"matrix_{vk_type.lower()}_{short_usage}_{row}x{col}.asm"
    full_path = output_directory.joinpath(shader_name)

    is_col_major = layout_name == "column"

    info = parse_assembly_from_file(full_path)

    for instr in info.values():
        if "MOVM" in instr:
            print((col, row, vk_type, is_col_major))
            print("MOVM in use, ignoring")
            return

    expected_mat_val_count = (col * row) // 32
    # (row, col, vk_type, is_col_major, short_usage, matrix_layout_name, user_data, orig_func)
    new_entry = (
        row,
        col,
        vk_type,
        is_col_major,
        short_usage,
        matrix_layout_name,
        (full_path, expected_mat_val_count, byte_size),
        exec_shader,
    )

    if new_entry not in TEST_CASES:
        TEST_CASES.append(new_entry)


def append_shader_tests(output_directory: Path, entry: Dict[str, object]):
    a_type = entry["a_type"]
    b_type = entry["b_type"]
    c_type = entry["c_type"]
    result_type = entry["result_type"]
    m_size = entry["m_size"]
    n_size = entry["n_size"]
    k_size = entry["k_size"]
    matrix_layout_name = f"{m_size}x{n_size}x{k_size}"

    for layout_name, store_layout in [
        ("row", "gl_CooperativeMatrixLayoutRowMajor"),
        ("column", "gl_CooperativeMatrixLayoutColumnMajor"),
    ]:
        final_output_directory = output_directory.joinpath(matrix_layout_name).joinpath(
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
            matrix_layout_name,
        )
        # KxN (B)
        add_shader_test(
            final_output_directory,
            b_type,
            k_size,
            n_size,
            "gl_MatrixUseB",
            layout_name,
            matrix_layout_name,
        )
        # MxN (C)
        add_shader_test(
            final_output_directory,
            c_type,
            m_size,
            n_size,
            "gl_MatrixUseAccumulator",
            layout_name,
            matrix_layout_name,
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
        "n_size": 8,
        "k_size": 32,
        "a_type": "UINT8",
        "b_type": "UINT8",
        "c_type": "UINT32",
        "result_type": "UINT32",
        "saturating_accumulation": 0,
    },
]

for entry in SUPPORTED_CFG_DEBUG:
    append_shader_tests(Path("./coop_matrix_layout_store_shaders/sm86"), entry)

print(f"selected_element = {selected_element}")
print(f"selected_stride = {selected_stride}")
print("========")

for test_case in TEST_CASES:
    (
        row,
        col,
        vk_type,
        is_col_major,
        short_usage,
        matrix_layout_name,
        user_data,
        orig_func,
    ) = test_case

    test_variant(
        row,
        col,
        selected_stride,
        selected_element,
        vk_type,
        is_col_major,
        short_usage,
        matrix_layout_name,
        user_data,
        orig_func,
    )


sys.exit(0)
