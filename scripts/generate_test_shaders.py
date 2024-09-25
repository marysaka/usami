import argparse
from pathlib import Path
import subprocess
from typing import Dict
from coop_matrix_defs import *

SHADER_STORE_TEMPLATE = """
// compiler_extensions: VK_KHR_cooperative_matrix
#version 450

#extension GL_GOOGLE_include_directive : require
#extension GL_EXT_shader_16bit_storage : require
#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_KHR_memory_scope_semantics : require
#extension GL_EXT_shader_explicit_arithmetic_types : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_cooperative_matrix : require

<template configuration>

layout(local_size_x_id = 0, local_size_y_id = 1, local_size_z = 1) in;
layout(set=0, binding=0) buffer Output { elementType x[]; } outputO;
layout(set=0, binding=1) buffer CustomStride { uint stride[]; } customStride;
layout(set=0, binding=3) buffer CustomElement { uint element[]; } customElement;

#define coopmatType coopmat<elementType, gl_ScopeSubgroup, ROW, COL, MATRIX_USAGE>

void main()
{
   uint element = customElement.element[0];
   uint stride = customStride.stride[0];
   coopmatType mat = coopmatType(1.0);

   barrier();

   coopMatStore(mat, outputO.x, element, stride, MATRIX_LAYOUT_MAJOR);
}
"""


def generate_shader(
    output_directory: Path,
    out: Dict[str, str],
    vk_type: str,
    row: int,
    col: int,
    usage: str,
    store_layout: str,
):
    element_type = VK_TYPE_TO_GLSL_TYPE[vk_type]
    short_usage = MATRIX_USAGE_SHORT_NAME[usage]
    shader_name = f"matrix_{vk_type.lower()}_{short_usage}_{row}x{col}.glsl"

    config = f"#define elementType {element_type}\n"
    config += f"#define MATRIX_LAYOUT_MAJOR {store_layout}\n"
    config += f"#define MATRIX_USAGE {usage}\n"
    config += f"const int ROW = {row};\n"
    config += f"const int COL = {col};\n"
    out[output_directory.joinpath(shader_name)] = (
        f"// {shader_name}\n"
        + SHADER_STORE_TEMPLATE.replace("<template configuration>", config)
    )


def append_shader_tests(
    output_directory: Path, out: Dict[str, str], entry: Dict[str, object]
):
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
        final_output_directory.mkdir(parents=True, exist_ok=True)

        # MxNxK (A/B/C/D)
        # MxK (A)
        generate_shader(
            final_output_directory,
            out,
            a_type,
            m_size,
            k_size,
            "gl_MatrixUseA",
            store_layout,
        )
        # KxN (B)
        generate_shader(
            final_output_directory,
            out,
            b_type,
            k_size,
            n_size,
            "gl_MatrixUseB",
            store_layout,
        )
        # MxN (C)
        generate_shader(
            final_output_directory,
            out,
            c_type,
            m_size,
            n_size,
            "gl_MatrixUseAccumulator",
            store_layout,
        )

        # MxN (D)
        if c_type != result_type:
            generate_shader(
                final_output_directory,
                out,
                result_type,
                m_size,
                n_size,
                "gl_MatrixUseAccumulator",
                store_layout,
            )

    pass


parser = argparse.ArgumentParser()
parser.add_argument("output_directory")
parser.add_argument("host")
parser.add_argument("--device-id", type=int, required=True)
parser.add_argument("--vendor-id", type=int, default=4318)
parser.add_argument("--sm", type=int, required=True)
args = parser.parse_args()

result: Dict[Path, str] = dict()
output_directory = Path(args.output_directory).absolute()
target_host = args.host
device_id = args.device_id
vendor_id = args.vendor_id
sm_ver = args.sm

if sm_ver == 75:
    cfg = SUPPORTED_CFGS_SM75
elif sm_ver == 86:
    cfg = SUPPORTED_CFGS_SM86
else:
    raise Exception("TODO")

for entry in cfg:
    append_shader_tests(output_directory, result, entry)


for path, shader_content in result.items():
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(shader_content)

# Generate binary outputs
for path, shader_content in result.items():
    spv_path = path.parent.joinpath(path.name.replace(".glsl", ".spv"))

    # First build SPIR-V
    args = [
        "glslangValidator",
        "-g0",
        "--target-env",
        "vulkan1.3",
        "-S",
        "comp",
        str(path),
        "-o",
        str(spv_path),
    ]
    subprocess.check_call(args)

    # Remote compile shader
    args = [
        "python3",
        "scripts/compile_shader.py",
        "--host",
        target_host,
        "--vendor-id",
        str(vendor_id),
        "--device-id",
        str(device_id),
        str(path.parent),
        str(spv_path),
    ]
    subprocess.check_call(args)

    # Generate assembly output
    sm_code_path = path.parent.joinpath(path.name.replace(".glsl", ".code"))
    sm_code_asm_path = path.parent.joinpath(path.name.replace(".glsl", ".asm"))
    args = [
        "nvdisasm",
        "-raw",
        "--binary",
        f"SM{sm_ver}",
        str(sm_code_path),
    ]
    sm_assembly = subprocess.check_output(args)
    sm_code_asm_path.write_bytes(sm_assembly)
