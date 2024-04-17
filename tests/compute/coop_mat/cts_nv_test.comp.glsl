#version 450

#extension GL_GOOGLE_include_directive : require
#extension GL_EXT_shader_16bit_storage : require
#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_KHR_memory_scope_semantics : require
#extension GL_EXT_shader_explicit_arithmetic_types : require
#extension GL_KHR_shader_subgroup_basic : require


// Values
#define COOP_TYPE_I 0
#define COOP_TYPE_U 1
#define COOP_TYPE_F 2

// Select if KHR ext should be used
#define USE_KHR_EXT

// Indicate the type of coop matrix
#define COOP_TYPE COOP_TYPE_F

#include "coop_framework.h"

// Shader configuration
const int ROW = 16;
const int COL = 8;

layout(set=0, binding=0) buffer Output { float16_t x[]; } outputO;
layout(set=0, binding=1) buffer CustomStride { uint stride[]; } customStride;
layout(set=0, binding=3) buffer CustomElement { uint element[]; } customElement;

#define coopmatType DEF_COOP_MAT_TYPE(float16_t, 16, ROW, COL, MATRIX_USE_ACCUMULATOR)

void main()
{
   uint element3 = customElement.element[0];
   uint strideO = customStride.stride[0]; //+ customStride.stride[1];
   coopmatType matO = coopmatType(1.0);
   matO[0] = 1.0hf;
   matO[1] = 2.0hf;
   matO[2] = 3.0hf;
   matO[3] = 4.0hf;

   // coopFrameworkMatStore(matO, outputO.x, element3, strideO, MATRIX_LAYOUT_ROW_MAJOR);
   coopFrameworkMatStore(matO, outputO.x, 0, 2, MATRIX_LAYOUT_ROW_MAJOR);
   // coopFrameworkMatStore(matO, outputO.x, element3, strideO, MATRIX_LAYOUT_COLUMN_MAJOR);
}