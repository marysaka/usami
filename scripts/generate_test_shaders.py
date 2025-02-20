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
   coopmatType mat = coopmatType(elementType(0));

   for (int i = 0; i < mat.length(); i++)
   {
      mat[i] = elementType(i + 1);
   }

   barrier();

   coopMatStore(mat, outputO.x, element, stride, MATRIX_LAYOUT_MAJOR);
}
"""


SHADER_MULADD_TEMPLATE = """
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
layout(set=0, binding=0) buffer Output { elementTypeD x[]; } outputO;
layout(set=0, binding=1) buffer CustomStride { uint stride[]; } customStride;
layout(set=0, binding=3) buffer CustomElement { uint element[]; } customElement;

#define coopmatTypeA coopmat<elementTypeA, gl_ScopeSubgroup, M, K, gl_MatrixUseA>
#define coopmatTypeB coopmat<elementTypeB, gl_ScopeSubgroup, K, N, gl_MatrixUseB>
#define coopmatTypeC coopmat<elementTypeC, gl_ScopeSubgroup, M, N, gl_MatrixUseAccumulator>
#define coopmatTypeD coopmat<elementTypeD, gl_ScopeSubgroup, M, N, gl_MatrixUseAccumulator>

void main()
{
   uint element = customElement.element[0];
   uint stride = customStride.stride[0];
   coopmatTypeA matA = coopmatTypeA(elementTypeA(0));
   coopmatTypeB matB = coopmatTypeB(elementTypeB(0));
   coopmatTypeC matC = coopmatTypeC(elementTypeC(0));

   for (int i = 0; i < matA.length(); i++)
   {
      matA[i] = elementTypeA((0xA << 4) | i + 1);
   }

   for (int i = 0; i < matB.length(); i++)
   {
      matB[i] = elementTypeB((0xB << 4) | i + 1);
   }

   for (int i = 0; i < matC.length(); i++)
   {
      matC[i] = elementTypeC((0xC << 4) | i + 1);
   }

   barrier();

   coopmatTypeD matD = coopMatMulAdd(matA, matB, matC);
   barrier();

   coopMatStore(matD, outputO.x, element, stride, gl_CooperativeMatrixLayoutRowMajor);
}
"""

SHADER_MULADD_FULL_TEMPLATE = """
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
layout(set=0, binding=0) buffer Output { elementTypeD x[]; } outputO;
layout(set=0, binding=1) buffer CustomStride { uint stride[]; } customStride;
layout(set=0, binding=3) buffer CustomElement { uint element[]; } customElement;

layout(set=1, binding = 0) readonly buffer a_blob { elementTypeA a_blob_data[]; };
layout(set=1, binding = 1) readonly buffer b_blob { elementTypeB b_blob_data[]; };
layout(set=1, binding = 2) readonly buffer c_blob { elementTypeC c_blob_data[]; };


#define coopmatTypeA coopmat<elementTypeA, gl_ScopeSubgroup, M, K, gl_MatrixUseA>
#define coopmatTypeB coopmat<elementTypeB, gl_ScopeSubgroup, K, N, gl_MatrixUseB>
#define coopmatTypeC coopmat<elementTypeC, gl_ScopeSubgroup, M, N, gl_MatrixUseAccumulator>
#define coopmatTypeD coopmat<elementTypeD, gl_ScopeSubgroup, M, N, gl_MatrixUseAccumulator>

shared elementTypeA tmp_a[M * K];
shared elementTypeB tmp_b[K * N];
shared elementTypeC tmp_c[M * N];

void main()
{
   const int gx = int(gl_GlobalInvocationID.x);
   const int lx = int(gl_LocalInvocationID.x);

   // Load some values (we really don't care here)
   if (lx < 32)
   {
       tmp_a[lx] = a_blob_data[gx];
       tmp_b[lx] = b_blob_data[gx];
       tmp_c[lx] = c_blob_data[gx];
   }

   barrier();

   coopmatTypeA matA;
   coopmatTypeB matB;
   coopmatTypeC matC;

   coopMatLoad(matA, tmp_a, 0, strideTypeA, MATRIX_LAYOUT_MAJOR_A);
   coopMatLoad(matB, tmp_b, 0, strideTypeB, MATRIX_LAYOUT_MAJOR_B);
   coopMatLoad(matC, tmp_c, 0, strideTypeC, MATRIX_LAYOUT_MAJOR_C);

   coopmatTypeD matD = coopMatMulAdd(matA, matB, matC);
   barrier();

   coopMatStore(matD, outputO.x, 0, strideTypeD, MATRIX_LAYOUT_MAJOR_D);
}
"""


def generate_shader_store(
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


def generate_shader_muladd(
    output_directory: Path,
    out: Dict[str, str],
    m: int,
    n: int,
    k: int,
    vk_type_a: str,
    vk_type_b: str,
    vk_type_c: str,
    vk_type_d: str,
):
    element_type_a = VK_TYPE_TO_GLSL_TYPE[vk_type_a]
    element_type_b = VK_TYPE_TO_GLSL_TYPE[vk_type_b]
    element_type_c = VK_TYPE_TO_GLSL_TYPE[vk_type_c]
    element_type_d = VK_TYPE_TO_GLSL_TYPE[vk_type_d]
    vk_types = f"{vk_type_a.lower()}_{vk_type_b.lower()}_{vk_type_c.lower()}_{vk_type_d.lower()}"
    shader_name = f"matrix_muladd_{vk_types}_{m}x{n}x{k}.glsl"
    config = f"#define elementTypeA {element_type_a}\n"
    config += f"#define elementTypeB {element_type_b}\n"
    config += f"#define elementTypeC {element_type_c}\n"
    config += f"#define elementTypeD {element_type_d}\n"
    config += f"const int M = {m};\n"
    config += f"const int N = {n};\n"
    config += f"const int K = {k};\n"
    out[output_directory.joinpath(shader_name)] = (
        f"// {shader_name}\n"
        + SHADER_MULADD_TEMPLATE.replace("<template configuration>", config)
    )


def generate_shader_muladd_full(
    output_directory: Path,
    out: Dict[str, str],
    m: int,
    n: int,
    k: int,
    vk_type_a: str,
    vk_type_b: str,
    vk_type_c: str,
    vk_type_d: str,
    store_layout_a: str,
    store_layout_b: str,
    store_layout_c: str,
    store_layout_d: str,
):
    element_type_a = VK_TYPE_TO_GLSL_TYPE[vk_type_a]
    element_type_b = VK_TYPE_TO_GLSL_TYPE[vk_type_b]
    element_type_c = VK_TYPE_TO_GLSL_TYPE[vk_type_c]
    element_type_d = VK_TYPE_TO_GLSL_TYPE[vk_type_d]

    element_size_a = VK_TYPE_TO_BYTE_SIZE[vk_type_a]
    element_size_b = VK_TYPE_TO_BYTE_SIZE[vk_type_b]
    element_size_c = VK_TYPE_TO_BYTE_SIZE[vk_type_c]
    element_size_d = VK_TYPE_TO_BYTE_SIZE[vk_type_d]

    vk_types = f"{vk_type_a.lower()}_{vk_type_b.lower()}_{vk_type_c.lower()}_{vk_type_d.lower()}"
    vk_types += f"_{store_layout_a.lower()}_{store_layout_b.lower()}_{store_layout_c.lower()}_{store_layout_d.lower()}"
    shader_name = f"matrix_muladd_full_{vk_types}_{m}x{n}x{k}.glsl"
    config = f"#define elementTypeA {element_type_a}\n"
    config += f"#define elementTypeB {element_type_b}\n"
    config += f"#define elementTypeC {element_type_c}\n"
    config += f"#define elementTypeD {element_type_d}\n"
    config += f"#define strideTypeA {element_size_a}\n"
    config += f"#define strideTypeB {element_size_b}\n"
    config += f"#define strideTypeC {element_size_c}\n"
    config += f"#define strideTypeD {element_size_d}\n"
    config += f"#define MATRIX_LAYOUT_MAJOR_A {store_layout_a}\n"
    config += f"#define MATRIX_LAYOUT_MAJOR_B {store_layout_b}\n"
    config += f"#define MATRIX_LAYOUT_MAJOR_C {store_layout_c}\n"
    config += f"#define MATRIX_LAYOUT_MAJOR_D {store_layout_d}\n"

    config += f"const int M = {m};\n"
    config += f"const int N = {n};\n"
    config += f"const int K = {k};\n"
    out[output_directory.joinpath(shader_name)] = (
        f"// {shader_name}\n"
        + SHADER_MULADD_FULL_TEMPLATE.replace("<template configuration>", config)
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

    matrix_output_directory = output_directory.joinpath(matrix_name)

    generate_shader_muladd(
        matrix_output_directory,
        out,
        m_size,
        n_size,
        k_size,
        a_type,
        b_type,
        c_type,
        result_type,
    )

    coop_matrix_layout = [
        "gl_CooperativeMatrixLayoutRowMajor",
        "gl_CooperativeMatrixLayoutColumnMajor",
    ]

    full_muladd_output_directory = matrix_output_directory.joinpath("full_muladd")
    full_muladd_output_directory.mkdir(parents=True, exist_ok=True)

    for store_layout_a in coop_matrix_layout:
        for store_layout_b in coop_matrix_layout:
            for store_layout_c in coop_matrix_layout:
                for store_layout_d in coop_matrix_layout:
                    generate_shader_muladd_full(
                        full_muladd_output_directory,
                        out,
                        m_size,
                        n_size,
                        k_size,
                        a_type,
                        b_type,
                        c_type,
                        result_type,
                        store_layout_a,
                        store_layout_b,
                        store_layout_c,
                        store_layout_d,
                    )

    for layout_name, store_layout in [
        ("row", "gl_CooperativeMatrixLayoutRowMajor"),
        ("column", "gl_CooperativeMatrixLayoutColumnMajor"),
    ]:
        final_output_directory = matrix_output_directory.joinpath(layout_name)
        final_output_directory.mkdir(parents=True, exist_ok=True)

        # MxNxK (A/B/C/D)
        # MxK (A)
        generate_shader_store(
            final_output_directory,
            out,
            a_type,
            m_size,
            k_size,
            "gl_MatrixUseA",
            store_layout,
        )
        # KxN (B)
        generate_shader_store(
            final_output_directory,
            out,
            b_type,
            k_size,
            n_size,
            "gl_MatrixUseB",
            store_layout,
        )
        # MxN (C)
        generate_shader_store(
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
            generate_shader_store(
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
