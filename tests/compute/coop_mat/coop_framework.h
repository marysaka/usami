// Values
#ifndef COOP_TYPE_I
#define COOP_TYPE_I 0
#endif

#ifndef COOP_TYPE_U
#define COOP_TYPE_U 1
#endif

#ifndef COOP_TYPE_F
#define COOP_TYPE_F 2
#endif

// Start of magical wrapper
#ifdef USE_KHR_EXT
// KHR EXT
#extension GL_KHR_cooperative_matrix : require

// Values
#define MATRIX_LAYOUT_ROW_MAJOR gl_CooperativeMatrixLayoutRowMajor
#define MATRIX_LAYOUT_COLUMN_MAJOR gl_CooperativeMatrixLayoutColumnMajor
#define MATRIX_USE_A gl_MatrixUseA
#define MATRIX_USE_B gl_MatrixUseB
#define MATRIX_USE_ACCUMULATOR gl_MatrixUseAccumulator

// Operations
#define DEF_COOP_MAT_TYPE(input_type, bit_size, rows, columns, usage)          \
  coopmat<input_type, gl_ScopeSubgroup, rows, columns, usage>
#define coopFrameworkMatLoad(dst, src, element, stride, column_major)        \
  coopMatLoad(dst, src, element, stride, column_major)
#define coopFrameworkMatStore(src, dst, element, stride, column_major)       \
  coopMatStore(src, dst, element, stride, column_major)
#define coopFrameworkMatMulAdd(a, b, c) coopMatMulAdd(a, b, c)

#else

// COOP_TYPE is needed
#ifndef COOP_TYPE
#error "COOP_TYPE needs to be defined when USE_KHR_EXT is not set"
#endif

// NVIDIA EXT
#extension GL_NV_cooperative_matrix : require
#extension GL_NV_integer_cooperative_matrix : require

// Values
#define MATRIX_LAYOUT_ROW_MAJOR false
#define MATRIX_LAYOUT_COLUMN_MAJOR true
#define MATRIX_USE_A 0
#define MATRIX_USE_B 1
#define MATRIX_USE_ACCUMULATOR 2

// Operations
#if COOP_TYPE == COOP_TYPE_F
#define DEF_COOP_MAT_TYPE(input_type, bit_size, rows, columns, usage)          \
  fcoopmatNV<bit_size, gl_ScopeSubgroup, rows, columns>
#elif COOP_TYPE == COOP_TYPE_U
#define DEF_COOP_MAT_TYPE(input_type, bit_size, rows, columns, usage)          \
  ucoopmatNV<bit_size, gl_ScopeSubgroup, rows, columns>
#elif COOP_TYPE == COOP_TYPE_I
#define DEF_COOP_MAT_TYPE(input_type, bit_size, rows, columns, usage)          \
  icoopmatNV<bit_size, gl_ScopeSubgroup, rows, columns>
#endif
#define coopFrameworkMatLoad(dst, src, element, stride, column_major)        \
  coopMatLoadNV(dst, src, element, stride, column_major)
#define coopFrameworkMatStore(src, dst, element, stride, column_major)       \
  coopMatStoreNV(src, dst, element, stride, column_major)
#define coopFrameworkMatMulAdd(a, b, c) coopMatMulAddNV(a, b, c)
#endif
