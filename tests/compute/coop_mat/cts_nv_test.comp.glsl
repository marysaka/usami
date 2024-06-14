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
// #define USE_KHR_EXT

// Indicate the type of coop matrix
#define COOP_TYPE COOP_TYPE_U

#include "coop_framework.h"

// MxNxK (A/B/C/D)
// MxK (A)
// KxN (B)
// MxN (C)
// MxN (D)

// 16x16x16 (F16/F16/F16/F16)
// 16x16 (F16)
// 16x16 (F16)
// 16x16 (F16)
// 16x16 (F16)

// 16x8x8 (F16/F16/F16/F16)
// 16x8 (F16)
// 8x8 (F16)
// 16x8 (F16)
// 16x8 (F16)

// 16x16x16 (F16/F16/F32/F32)
// 16x16 (F16)
// 16x16 (F16)
// 16x16 (F32)
// 16x16 (F32)

// 16x8x16 (F16/F16/F32/F32)
// 16x16 (F16)
// 16x8 (F16)
// 16x8 (F32)
// 16x8 (F32)

// 16x8x8 (F16/F16/F32/F32)
// 16x8 (F16)
// 8x8 (F16)
// 16x8 (F32)
// 16x8 (F32)

// 16x16x32 (U8/U8/U32/U32)
// 16x32 (U8)
// 32x16 (U8)
// 16x16 (U32)
// 16x16 (U32)


// 16x16x32 (S8/S8/S32/S32)
// 16x32 (S8)
// 32x16 (S8)
// 16x16 (S32)
// 16x16 (S32)

// 16x8x32 (U8/U8/U32/U32)
// 16x32 (U8)
// 32x8 (U8)
// 16x8 (U32)
// 16x8 (U32)

// 16x8x32 (S8/S8/S32/S32)
// 16x32 (S8)
// 32x8 (S8)
// 16x8 (S32)
// 16x8 (S32)

// Shader configuration

const int ROW = 32;
const int COL = 8;
#define elementType uint8_t
const int BIT_SIZE = 8;
// #define matrix_layout_major MATRIX_LAYOUT_COLUMN_MAJOR
#define matrix_layout_major MATRIX_LAYOUT_ROW_MAJOR

layout(local_size_x_id = 0, local_size_y_id = 1, local_size_z = 1) in;

layout(set=0, binding=0) buffer Output { elementType x[]; } outputO;
layout(set=0, binding=1) buffer CustomStride { uint stride[]; } customStride;
layout(set=0, binding=3) buffer CustomElement { uint element[]; } customElement;

#define coopmatType DEF_COOP_MAT_TYPE(elementType, BIT_SIZE, ROW, COL, MATRIX_USE_ACCUMULATOR)

void main()
{
   uint element3 = customElement.element[0];
   uint strideO = customStride.stride[0]; //+ customStride.stride[1];
   coopmatType matO = coopmatType(1.0);

   //for (int i = 0; i < matO.length(); i++) {
   //   matO[i] = elementType(i);
   //}

   barrier();

   coopFrameworkMatStore(matO, outputO.x, element3, strideO, matrix_layout_major);
}