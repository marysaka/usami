// matrix_muladd_float16_float16_float32_float32_16x8x16.glsl

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
const int M = 16;
const int N = 8;
const int K = 16;


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
