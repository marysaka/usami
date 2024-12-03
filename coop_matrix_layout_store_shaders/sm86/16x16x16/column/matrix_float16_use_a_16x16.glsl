// matrix_float16_use_a_16x16.glsl

// compiler_extensions: VK_KHR_cooperative_matrix
#version 450

#extension GL_GOOGLE_include_directive : require
#extension GL_EXT_shader_16bit_storage : require
#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_KHR_memory_scope_semantics : require
#extension GL_EXT_shader_explicit_arithmetic_types : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_cooperative_matrix : require

#define elementType float16_t
#define MATRIX_LAYOUT_MAJOR gl_CooperativeMatrixLayoutColumnMajor
#define MATRIX_USAGE gl_MatrixUseA
const int ROW = 16;
const int COL = 16;


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
