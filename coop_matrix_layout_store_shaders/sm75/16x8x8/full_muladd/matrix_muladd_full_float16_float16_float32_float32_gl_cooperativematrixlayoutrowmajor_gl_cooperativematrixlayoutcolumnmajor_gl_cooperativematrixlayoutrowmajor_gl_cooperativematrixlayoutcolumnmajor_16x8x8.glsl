// matrix_muladd_full_float16_float16_float32_float32_gl_cooperativematrixlayoutrowmajor_gl_cooperativematrixlayoutcolumnmajor_gl_cooperativematrixlayoutrowmajor_gl_cooperativematrixlayoutcolumnmajor_16x8x8.glsl

// compiler_extensions: VK_KHR_cooperative_matrix
#version 450

#extension GL_GOOGLE_include_directive : require
#extension GL_EXT_shader_16bit_storage : require
#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_KHR_memory_scope_semantics : require
#extension GL_EXT_shader_explicit_arithmetic_types : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_cooperative_matrix : require

#define elementTypeA float16_t
#define elementTypeB float16_t
#define elementTypeC float32_t
#define elementTypeD float32_t
#define strideTypeA 2
#define strideTypeB 2
#define strideTypeC 4
#define strideTypeD 4
#define MATRIX_LAYOUT_MAJOR_A gl_CooperativeMatrixLayoutRowMajor
#define MATRIX_LAYOUT_MAJOR_B gl_CooperativeMatrixLayoutColumnMajor
#define MATRIX_LAYOUT_MAJOR_C gl_CooperativeMatrixLayoutRowMajor
#define MATRIX_LAYOUT_MAJOR_D gl_CooperativeMatrixLayoutColumnMajor
const int M = 16;
const int N = 8;
const int K = 8;


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
