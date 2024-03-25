#version 450

#extension GL_EXT_shader_16bit_storage : require
#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_KHR_memory_scope_semantics : require
#extension GL_EXT_shader_explicit_arithmetic_types : require

// Values
#define COOP_TYPE_I 0
#define COOP_TYPE_U 1
#define COOP_TYPE_F 2

// Select if KHR ext should be used
#define USE_KHR_EXT 1

// Indicate the type of coop matrix
#define COOP_TYPE COOP_TYPE_F

// Configuration of the shader

// Matrix setup
#define M_SIZE 16
#define N_SIZE 16
#define K_SIZE 16
// Shared
#define inputTypeBase float16_t
#define INPUT_BIT_SIZE 16

#define inputTypeA inputTypeBase
#define A_BIT_SIZE INPUT_BIT_SIZE

#define inputTypeB inputTypeBase
#define B_BIT_SIZE INPUT_BIT_SIZE

#define inputTypeC inputTypeBase
#define C_BIT_SIZE INPUT_BIT_SIZE

#define inputTypeD inputTypeBase
#define D_BIT_SIZE INPUT_BIT_SIZE

// Start of magical wrapper
#if USE_KHR_EXT == 1
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
#define CREATE_COOP_MAT(type, dst, src, stride, bit_size, column_major)        \
  type dst;                                                                    \
  coopMatLoad(dst, src, stride, bit_size / 8, column_major)
#define STORE_COOP_MAT(dst, src, stride, bit_size, column_major)               \
  coopMatStore(src, dst, stride, bit_size / 8, column_major)
#define COOP_MAT_MUL_ADD(a, b, c) coopMatMulAdd(a, b, c)

#else
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
#define CREATE_COOP_MAT(type, dst, src, stride, bit_size, column_major)        \
  type dst;                                                                    \
  coopMatLoadNV(dst, src, stride, bit_size / 8, column_major)
#define STORE_COOP_MAT(dst, src, stride, bit_size, column_major)               \
  coopMatStoreNV(src, dst, stride, bit_size / 8, column_major)
#define COOP_MAT_MUL_ADD(a, b, c) coopMatMulAddNV(a, b, c)
#endif
// end of magical wrapper

// Shader start
#define coopmatTypeA                                                           \
  DEF_COOP_MAT_TYPE(inputTypeA, A_BIT_SIZE, M_SIZE, K_SIZE, MATRIX_USE_A)
#define coopmatTypeB                                                           \
  DEF_COOP_MAT_TYPE(inputTypeB, B_BIT_SIZE, K_SIZE, N_SIZE, MATRIX_USE_B)
#define coopmatTypeC                                                           \
  DEF_COOP_MAT_TYPE(inputTypeC, C_BIT_SIZE, M_SIZE, N_SIZE,                    \
                    MATRIX_USE_ACCUMULATOR)
#define coopmatTypeD                                                           \
  DEF_COOP_MAT_TYPE(inputTypeD, D_BIT_SIZE, M_SIZE, N_SIZE,                    \
                    MATRIX_USE_ACCUMULATOR)

layout(constant_id = 0) const int loop = 1;

layout(binding = 0) readonly buffer blob {
  inputTypeA a_blob_data[M_SIZE * K_SIZE];
  inputTypeB b_blob_data[K_SIZE * N_SIZE];
  inputTypeB c_blob_data[M_SIZE * N_SIZE];
};
layout(binding = 3) writeonly buffer d_blob { inputTypeD d_blob_data[]; };

shared inputTypeA tmp_a[M_SIZE * K_SIZE];
shared inputTypeB tmp_b[K_SIZE * N_SIZE];
shared inputTypeC tmp_c[M_SIZE * N_SIZE];
shared inputTypeD tmp_d[M_SIZE * N_SIZE];

void main() {
  const int gx = int(gl_GlobalInvocationID.x);
  const int lx = int(gl_LocalInvocationID.x);

  // This is only to write some values around to ensure the shared are used.
  // This is of course not correct but will do for now
  if (lx < 32) {
    tmp_a[lx] = a_blob_data[gx];
    tmp_b[lx] = b_blob_data[gx];
    tmp_c[lx] = c_blob_data[gx];
  }

  barrier();

  // LOAD
  CREATE_COOP_MAT(coopmatTypeA, a, tmp_a, 0, A_BIT_SIZE,
                  MATRIX_LAYOUT_ROW_MAJOR);
  CREATE_COOP_MAT(coopmatTypeB, b, tmp_b, 0, B_BIT_SIZE,
                  MATRIX_LAYOUT_ROW_MAJOR);
  CREATE_COOP_MAT(coopmatTypeC, c, tmp_c, 0, C_BIT_SIZE,
                  MATRIX_LAYOUT_ROW_MAJOR);

  // COOP_MAT_MUL_ADD
  coopmatTypeD d = COOP_MAT_MUL_ADD(a, b, c);

  // STORE
  STORE_COOP_MAT(tmp_d, d, 4, D_BIT_SIZE, MATRIX_LAYOUT_ROW_MAJOR);
  barrier();

  if (lx < 32) {
    // Some random external write
    d_blob_data[gx] = tmp_d[lx];
  }
}

// m_size: 16, n_size: 16, k_size: 16, a_type: FLOAT16, b_type: FLOAT16, c_type: FLOAT16, result_type: FLOAT16
// LOAD:
// /*0150*/                   LDSM.16.M88.4 R4, [R3+0x600] ;                         /* 0x000600000304783b */
//                                                                                   /* 0x001fe20000000200 */
// ...
// /*0180*/                   LDSM.16.MT88.4 R8, [R3+0x400] ;                        /* 0x000400000308783b */
//                                                                                   /* 0x000fe20000004200 */
// ...
// /*01c0*/                   LDSM.16.M88.4 R12, [R3+0x200] ;                        /* 0x00020000030c783b */
//                                                                                   /* 0x000e240000000200 */
//
// COOP_MAT_MUL_ADD:
// /*01d0*/                   HMMA.16816.F16 R8, R4, R8, R12 ;                       /* 0x000000080408723c */
//                                                                                   /* 0x001f5e000000080c */
// ...
// /*01f0*/                   HMMA.16816.F16 R10, R4, R10, R14 ;                     /* 0x0000000a040a723c */
//                                                                                   /* 0x000f6c000000080e */
//
// STORE:
// /*0200*/                   STS [R1], R8 ;                                         /* 0x0000000801007388 */
//                                                                                   /* 0x0201e80000000800 */
// /*0210*/                   STS [R1+0x20], R9 ;                                    /* 0x0000200901007388 */
//                                                                                   /* 0x0001d80000000800 */
// /*0220*/                   STS [R1+0x10], R10 ;                                   /* 0x0000100a01007388 */
//                                                                                   /* 0x0001e80000000800 */
// /*0230*/                   STS [R1+0x30], R11 ;                                   /* 0x0000300b01007388 */


// m_size: 16, n_size: 8, k_size: 16, a_type: FLOAT16, b_type: FLOAT16, c_type: FLOAT16, result_type: FLOAT16
// LOAD:
// /*0160*/                   LDSM.16.MT88.2 R2, [R8+0x200] ;                        /* 0x000200000802783b */
//                                                                                   /* 0x001fe20000004100 */
// ...
// /*0180*/                   LDSM.16.M88.2 R10, [R8+0x100] ;                        /* 0x00010000080a783b */
//                                                                                   /* 0x000fe80000000100 */
// ...
// /*0190*/                   LDSM.16.M88.4 R4, [R4+0x300] ;                         /* 0x000300000404783b */
//                                                                                   /* 0x000e240000000200 */
//
// COOP_MAT_MUL_ADD:
// /*01a0*/                   HMMA.16816.F16 R2, R4, R2, R10 ;                       /* 0x000000020402723c */
//                                                                                   /* 0x001b5a000000080a */
//
// STORE:
// /*01f0*/                   STS [R1], R2 ;                                         /* 0x0000000201007388 */
//                                                                                   /* 0x0001e80000000800 */
// /*0200*/                   STS [R1+0x20], R3 ;                                    /* 0x0000200301007388 */
//                                                                                   /* 0x0001e20000000800 */


// m_size: 16, n_size: 8, k_size: 8, a_type: FLOAT16, b_type: FLOAT16, c_type: FLOAT16, result_type: FLOAT16
// LOAD:
// /*0130*/                   LDSM.16.MT88 R1, [R4+0x200] ;                          /* 0x000200000401783b */
//                                                                                   /* 0x001fe80000004000 */
// /*0140*/                   LDSM.16.M88.2 R2, [R4+0x280] ;                         /* 0x000280000402783b */
//                                                                                   /* 0x000fe80000000100 */
// /*0150*/                   LDSM.16.M88.2 R6, [R4+0x100] ;                         /* 0x000100000406783b */
//                                                                                   /* 0x000e240000000100 */
//
// COOP_MAT_MUL_ADD:
// /*0160*/                   HMMA.1688.F16 R6, R2, R1, R6 ;                         /* 0x000000010206723c */
//                                                                                   /* 0x001b6a0000000006 */
//
// STORE:
// /*01c0*/                   STS [R1], R6 ;                                         /* 0x0000000601007388 */
//                                                                                   /* 0x0001e80000000800 */
// /*01d0*/                   STS [R1+0x20], R7 ;                                    /* 0x0000200701007388 */
//                                                                                   /* 0x0001e20000000800 */



// m_size: 16, n_size: 16, k_size: 16, a_type: FLOAT16, b_type: FLOAT16, c_type: FLOAT32, result_type: FLOAT32
// LOAD:
// /*01d0*/                   LDSM.16.M88.4 R8, [R3+0xa00] ;                         /* 0x000a00000308783b */
//                                                                                   /* 0x001ff00000000200 */
// /*01e0*/                   LDSM.16.MT88.4 R12, [R3+0x800] ;                       /* 0x00080000030c783b */
//                                                                                   /* 0x000e300000004200 */
// /*01f0*/                   LDS.U.64 R20, [R7.X8+0x420] ;                          /* 0x0004200007147984 */
//                                                                                   /* 0x000fe80000009a00 */
// /*0200*/                   LDS.U.64 R22, [R7.X8+0x4a0] ;                          /* 0x0004a00007167984 */
//                                                                                   /* 0x000e620000009a00 */
// COOP_MAT_MUL_ADD:
// /*0210*/                   HMMA.16816.F32 R0, R8, R12, R16 ;                      /* 0x0000000c0800723c */
//                                                                                   /* 0x001f5e0000001810 */
// ...
// /*0230*/                   HMMA.16816.F32 R12, R8, R14, R20 ;                     /* 0x0000000e080c723c */
//                                                                                   /* 0x002f6c0000001814 */
// STORE:
// /*0240*/                   STS.64 [R7.X8], R0 ;                                   /* 0x0000000007007388 */
//                                                                                   /* 0x0201e80000008a00 */
// /*0250*/                   STS.64 [R7.X8+0x80], R2 ;                              /* 0x0000800207007388 */
//                                                                                   /* 0x0003d80000008a00 */
// /*0260*/                   IMAD.MOV.U32 R0, RZ, RZ, R12 ;                         /* 0x000000ffff007224 */
//                                                                                   /* 0x001fe400078e000c */
// /*0270*/                   IMAD.MOV.U32 R1, RZ, RZ, R13 ;                         /* 0x000000ffff017224 */
//                                                                                   /* 0x000fe400078e000d */
// /*0280*/                   IMAD.MOV.U32 R2, RZ, RZ, R14 ;                         /* 0x000000ffff027224 */
//                                                                                   /* 0x002fe400078e000e */
// /*0290*/                   IMAD.MOV.U32 R3, RZ, RZ, R15 ;                         /* 0x000000ffff037224 */
//                                                                                   /* 0x000fc800078e000f */
// /*02a0*/                   STS.64 [R7.X8+0x20], R0 ;                              /* 0x0000200007007388 */
//                                                                                   /* 0x0001e80000008a00 */
// /*02b0*/                   STS.64 [R7.X8+0xa0], R2 ;                              /* 0x0000a00207007388 */


// m_size: 16, n_size: 8, k_size: 16, a_type: FLOAT16, b_type: FLOAT16, c_type: FLOAT32, result_type: FLOAT32
// LOAD:
// /*01a0*/                   LDSM.16.MT88.2 R6, [R6+0x400] ;                        /* 0x000400000606783b */
//                                                                                   /* 0x000fe80000004100 */
// /*01b0*/                   LDS.U.64 R8, [R13.X8+0x200] ;                          /* 0x000200000d087984 */
//                                                                                   /* 0x000fe80000009a00 */
// /*01c0*/                   LDS.U.64 R10, [R13.X8+0x280] ;                         /* 0x000280000d0a7984 */
//                                                                                   /* 0x000fe80000009a00 */
// /*01d0*/                   LDSM.16.M88.4 R0, [R5+0x500] ;                         /* 0x000500000500783b */
//                                                                                   /* 0x001e240000000200 */
// COOP_MAT_MUL_ADD:
// /*01e0*/                   HMMA.16816.F32 R8, R0, R6, R8 ;                        /* 0x000000060008723c */
//                                                                                   /* 0x001b5a0000001808 */
// STORE:
// /*0210*/                   IMAD.MOV.U32 R0, RZ, RZ, R8 ;                          /* 0x000000ffff007224 */
//                                                                                   /* 0x000fe400078e0008 */
// /*0220*/                   IMAD.MOV.U32 R1, RZ, RZ, R9 ;                          /* 0x000000ffff017224 */
//                                                                                   /* 0x000fe400078e0009 */
// /*0230*/                   IMAD.MOV.U32 R2, RZ, RZ, R10 ;                         /* 0x000000ffff027224 */
//                                                                                   /* 0x000fe400078e000a */
// /*0240*/                   IMAD.MOV.U32 R3, RZ, RZ, R11 ;                         /* 0x000000ffff037224 */
//                                                                                   /* 0x000fc800078e000b */
// /*0250*/                   STS.64 [R13.X8], R0 ;                                  /* 0x000000000d007388 */
//                                                                                   /* 0x0001e80000008a00 */
// /*0260*/                   STS.64 [R13.X8+0x80], R2 ;                             /* 0x000080020d007388 */

// m_size: 16, n_size: 8, k_size: 8, a_type: FLOAT16, b_type: FLOAT16, c_type: FLOAT32, result_type: FLOAT32
// LOAD:
// /*0170*/                   LDSM.16.MT88 R2, [R5+0x400] ;                          /* 0x000400000502783b */
//                                                                                   /* 0x001fe80000004000 */
// /*0180*/                   LDSM.16.M88.2 R0, [R5+0x480] ;                         /* 0x000480000500783b */
//                                                                                   /* 0x000fe80000000100 */
// /*0190*/                   LDS.U.64 R8, [R13.X8+0x200] ;                          /* 0x000200000d087984 */
//                                                                                   /* 0x000fe80000009a00 */
// /*01a0*/                   LDS.U.64 R10, [R13.X8+0x280] ;                         /* 0x000280000d0a7984 */
//                                                                                   /* 0x000e240000009a00 */
// COOP_MAT_MUL_ADD:
// /*01b0*/                   HMMA.1688.F32 R8, R0, R2, R8 ;                         /* 0x000000020008723c */
//                                                                                   /* 0x001b6a0000001008 */
// STORE:
// /*01e0*/                   IMAD.MOV.U32 R0, RZ, RZ, R8 ;                          /* 0x000000ffff007224 */
//                                                                                   /* 0x000fe400078e0008 */
// /*01f0*/                   IMAD.MOV.U32 R1, RZ, RZ, R9 ;                          /* 0x000000ffff017224 */
//                                                                                   /* 0x000fe400078e0009 */
// /*0200*/                   IMAD.MOV.U32 R2, RZ, RZ, R10 ;                         /* 0x000000ffff027224 */
//                                                                                   /* 0x000fe400078e000a */
// /*0210*/                   IMAD.MOV.U32 R3, RZ, RZ, R11 ;                         /* 0x000000ffff037224 */
//                                                                                   /* 0x000fc800078e000b */
// /*0220*/                   STS.64 [R13.X8], R0 ;                                  /* 0x000000000d007388 */
//                                                                                   /* 0x0001e80000008a00 */
// /*0230*/                   STS.64 [R13.X8+0x80], R2 ;                             /* 0x000080020d007388 */
//                                                                                   /* 0x0001e20000008a00 */

// m_size: 16, n_size: 16, k_size: 32, a_type: UINT8, b_type: UINT8, c_type: UINT32, result_type: UINT32
// RAW:
// /*0150*/                   LDS.U.S8 R18, [R7+0x803] ;                             /* 0x0008030007127984 */
//                                                                                   /* 0x000e620000001200 */
// /*0160*/                   LOP3.LUT R0, R8.reuse, 0xf, RZ, 0xc0, !PT ;            /* 0x0000000f08007812 */
//                                                                                   /* 0x040fe200078ec0ff */
// /*0170*/                   IMAD R33, R5, 0x2, R6 ;                                /* 0x0000000205217824 */
//                                                                                   /* 0x000fe200078e0206 */
// /*0180*/                   LOP3.LUT R1, R8, 0x10, RZ, 0xc0, !PT ;                 /* 0x0000001008017812 */
//                                                                                   /* 0x001fe200078ec0ff */
// /*0190*/                   LDS.U.S8 R14, [R7+0x802] ;                             /* 0x00080200070e7984 */
//                                                                                   /* 0x000e280000001200 */
// /*01a0*/                   LDS.U.S8 R19, [R7+0x80b] ;                             /* 0x00080b0007137984 */
//                                                                                   /* 0x000ea20000001200 */
// /*01b0*/                   IMAD.IADD R0, R0, 0x1, R1 ;                            /* 0x0000000100007824 */
//                                                                                   /* 0x000fc600078e0201 */
// /*01c0*/                   LDS.U.S8 R10, [R7+0x801] ;                             /* 0x00080100070a7984 */
//                                                                                   /* 0x000ee80000001200 */
// /*01d0*/                   LDS.U.S8 R15, [R7+0x80a] ;                             /* 0x00080a00070f7984 */
//                                                                                   /* 0x000f280000001200 */
// /*01e0*/                   LDS.U.S8 R20, [R7+0x813] ;                             /* 0x0008130007147984 */
//                                                                                   /* 0x000f680000001200 */
// /*01f0*/                   LDS.U.S8 R5, [R7+0x800] ;                              /* 0x0008000007057984 */
//                                                                                   /* 0x000f680000001200 */
// /*0200*/                   LDS.U.S8 R16, [R7+0x812] ;                             /* 0x0008120007107984 */
//                                                                                   /* 0x000f680000001200 */
// /*0210*/                   LDS.U.S8 R11, [R7+0x809] ;                             /* 0x00080900070b7984 */
//                                                                                   /* 0x000f680000001200 */
// /*0220*/                   LDS.U.S8 R12, [R7+0x811] ;                             /* 0x00081100070c7984 */
//                                                                                   /* 0x000f680000001200 */
// /*0230*/                   LDS.U.S8 R21, [R7+0x81b] ;                             /* 0x00081b0007157984 */
//                                                                                   /* 0x000f620000001200 */
// /*0240*/                   LOP3.LUT R23, R18, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff12177812 */
//                                                                                   /* 0x002fc600078ec0ff */
// /*0250*/                   LDS.U.64 R24, [R33.X8+0x400] ;                         /* 0x0004000021187984 */
//                                                                                   /* 0x000fe20000009a00 */
// /*0260*/                   PRMT R23, R23, 0x2104, R14 ;                           /* 0x0000210417177816 */
//                                                                                   /* 0x001fc6000000000e */
// /*0270*/                   LDSM.16.M88.4 R0, [R0+0xa00] ;                         /* 0x000a00000000783b */
//                                                                                   /* 0x000e220000000200 */
// /*0280*/                   LOP3.LUT R14, R19, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff130e7812 */
//                                                                                   /* 0x004fe400078ec0ff */
// /*0290*/                   PRMT R10, R23, 0x2104, R10 ;                           /* 0x00002104170a7816 */
//                                                                                   /* 0x008fca000000000a */
// /*02a0*/                   LDS.U.S8 R6, [R7+0x808] ;                              /* 0x0008080007067984 */
//                                                                                   /* 0x000e620000001200 */
// /*02b0*/                   PRMT R14, R14, 0x2104, R15 ;                           /* 0x000021040e0e7816 */
//                                                                                   /* 0x010fc6000000000f */
// /*02c0*/                   LDS.U.64 R26, [R33.X8+0x480] ;                         /* 0x00048000211a7984 */
//                                                                                   /* 0x000ea20000009a00 */
// /*02d0*/                   LOP3.LUT R15, R20, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff140f7812 */
//                                                                                   /* 0x020fc600078ec0ff */
// /*02e0*/                   LDS.U.S8 R8, [R7+0x810] ;                              /* 0x0008100007087984 */
//                                                                                   /* 0x000ee20000001200 */
// /*02f0*/                   PRMT R5, R10, 0x2104, R5 ;                             /* 0x000021040a057816 */
//                                                                                   /* 0x000fc60000000005 */
// /*0300*/                   LDS.U.S8 R17, [R7+0x81a] ;                             /* 0x00081a0007117984 */
//                                                                                   /* 0x000f220000001200 */
// /*0310*/                   PRMT R15, R15, 0x2104, R16 ;                           /* 0x000021040f0f7816 */
//                                                                                   /* 0x000fc60000000010 */
// /*0320*/                   LDS.U.S8 R13, [R7+0x819] ;                             /* 0x00081900070d7984 */
//                                                                                   /* 0x000f620000001200 */
// /*0330*/                   PRMT R11, R14, 0x2104, R11 ;                           /* 0x000021040e0b7816 */
//                                                                                   /* 0x000fc6000000000b */
// /*0340*/                   LDS.U.64 R28, [R33.X8+0x420] ;                         /* 0x00042000211c7984 */
//                                                                                   /* 0x000f620000009a00 */
// /*0350*/                   PRMT R15, R15, 0x2104, R12 ;                           /* 0x000021040f0f7816 */
//                                                                                   /* 0x000fc6000000000c */
// /*0360*/                   LDS.U.64 R30, [R33.X8+0x4a0] ;                         /* 0x0004a000211e7984 */
//                                                                                   /* 0x000f620000009a00 */
// /*0370*/                   LOP3.LUT R10, R21, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff150a7812 */
//                                                                                   /* 0x000fc600078ec0ff */
// /*0380*/                   LDS.U.S8 R9, [R7+0x818] ;                              /* 0x0008180007097984 */
//                                                                                   /* 0x000f620000001200 */
// /*0390*/                   IMMA.8816.U8.U8 R24, R0.ROW, R5.COL, R24 ;             /* 0x0000000500187237 */
//                                                                                   /* 0x001fe20000000418 */
// /*03a0*/                   PRMT R6, R11, 0x2104, R6 ;                             /* 0x000021040b067816 */
//                                                                                   /* 0x002fc60000000006 */
// /*03b0*/                   IMMA.8816.U8.U8 R26, R1.ROW, R5.COL, R26 ;             /* 0x00000005011a7237 */
//                                                                                   /* 0x004e22000000041a */
// /*03c0*/                   PRMT R8, R15, 0x2104, R8 ;                             /* 0x000021040f087816 */
//                                                                                   /* 0x008fe40000000008 */
// /*03d0*/                   PRMT R10, R10, 0x2104, R17 ;                           /* 0x000021040a0a7816 */
//                                                                                   /* 0x010fc80000000011 */
// /*03e0*/                   PRMT R10, R10, 0x2104, R13 ;                           /* 0x000021040a0a7816 */
//                                                                                   /* 0x020fc6000000000d */
// /*03f0*/                   IMMA.8816.U8.U8 R26, R3.ROW, R8.COL, R26 ;             /* 0x00000008031a7237 */
//                                                                                   /* 0x001fe8000000041a */
// /*0400*/                   IMMA.8816.U8.U8 R28, R0.ROW, R6.reuse.COL, R28 ;       /* 0x00000006001c7237 */
//                                                                                   /* 0x080e28000000041c */
// /*0410*/                   IMMA.8816.U8.U8 R30, R1.ROW, R6.COL, R30 ;             /* 0x00000006011e7237 */
//                                                                                   /* 0x000fe2000000041e */
// /*0420*/                   PRMT R9, R10, 0x2104, R9 ;                             /* 0x000021040a097816 */
//                                                                                   /* 0x000fc60000000009 */
// /*0430*/                   IMMA.8816.U8.U8 R0, R2.reuse.ROW, R8.COL, R24 ;        /* 0x0000000802007237 */
//                                                                                   /* 0x040e6a0000000418 */
// /*0440*/                   IMMA.8816.U8.U8 R28, R2.ROW, R9.COL, R28 ;             /* 0x00000009021c7237 */
//                                                                                   /* 0x0010a4000000041c */
// /*0450*/                   IADD3 R2, P0, R4, 0x4, RZ ;                            /* 0x0000000404027810 */
//                                                                                   /* 0x001fc60007f1e0ff */
// /*0460*/                   STS.64 [R33.X8], R0 ;                                  /* 0x0000000021007388 */
//                                                                                   /* 0x0021e20000008a00 */
// /*0470*/                   IMMA.8816.U8.U8 R30, R3.ROW, R9.COL, R30 ;             /* 0x00000009031e7237 */
//                                                                                   /* 0x000e62000000041e */
// /*0480*/                   ISETP.LE.U32.AND P0, PT, R2, c[0x0][0x68], !P0 ;       /* 0x00001a0002007a0c */
//                                                                                   /* 0x000fe20004703070 */
// /*0490*/                   IMAD.MOV.U32 R0, RZ, RZ, R26 ;                         /* 0x000000ffff007224 */
//                                                                                   /* 0x001fe400078e001a */
// /*04a0*/                   IMAD.MOV.U32 R1, RZ, RZ, R27 ;                         /* 0x000000ffff017224 */
//                                                                                   /* 0x000fd000078e001b */
// /*04b0*/                   STS.64 [R33.X8+0x80], R0 ;                             /* 0x0000800021007388 */
//                                                                                   /* 0x0005e40000008a00 */
// /*04c0*/                   IMAD.MOV.U32 R0, RZ, RZ, R28 ;                         /* 0x000000ffff007224 */
//                                                                                   /* 0x004fe400078e001c */
// /*04d0*/                   IMAD.MOV.U32 R1, RZ, RZ, R29 ;                         /* 0x000000ffff017224 */
//                                                                                   /* 0x000fd000078e001d */
// /*04e0*/                   STS.64 [R33.X8+0x20], R0 ;                             /* 0x0000200021007388 */
//                                                                                   /* 0x0003e40000008a00 */
// /*04f0*/                   IMAD.MOV.U32 R0, RZ, RZ, R30 ;                         /* 0x000000ffff007224 */
//                                                                                   /* 0x002fe400078e001e */
// /*0500*/                   IMAD.MOV.U32 R1, RZ, RZ, R31 ;                         /* 0x000000ffff017224 */
//                                                                                   /* 0x000fd000078e001f */
// /*0510*/                   STS.64 [R33.X8+0xa0], R0 ;                             /* 0x0000a00021007388 */

// m_size: 16, n_size: 16, k_size: 32, a_type: SINT8, b_type: SINT8, c_type: SINT32, result_type: SINT32
// RAW:
// /*0150*/                   LDS.U.S8 R18, [R7+0x803] ;                             /* 0x0008030007127984 */
//                                                                                   /* 0x000e620000001200 */
// /*0160*/                   LOP3.LUT R0, R8.reuse, 0xf, RZ, 0xc0, !PT ;            /* 0x0000000f08007812 */
//                                                                                   /* 0x040fe200078ec0ff */
// /*0170*/                   IMAD R33, R5, 0x2, R6 ;                                /* 0x0000000205217824 */
//                                                                                   /* 0x000fe200078e0206 */
// /*0180*/                   LOP3.LUT R1, R8, 0x10, RZ, 0xc0, !PT ;                 /* 0x0000001008017812 */
//                                                                                   /* 0x001fe200078ec0ff */
// /*0190*/                   LDS.U.S8 R14, [R7+0x802] ;                             /* 0x00080200070e7984 */
//                                                                                   /* 0x000e280000001200 */
// /*01a0*/                   LDS.U.S8 R19, [R7+0x80b] ;                             /* 0x00080b0007137984 */
//                                                                                   /* 0x000ea20000001200 */
// /*01b0*/                   IMAD.IADD R0, R0, 0x1, R1 ;                            /* 0x0000000100007824 */
//                                                                                   /* 0x000fc600078e0201 */
// /*01c0*/                   LDS.U.S8 R10, [R7+0x801] ;                             /* 0x00080100070a7984 */
//                                                                                   /* 0x000ee80000001200 */
// /*01d0*/                   LDS.U.S8 R15, [R7+0x80a] ;                             /* 0x00080a00070f7984 */
//                                                                                   /* 0x000f280000001200 */
// /*01e0*/                   LDS.U.S8 R20, [R7+0x813] ;                             /* 0x0008130007147984 */
//                                                                                   /* 0x000f680000001200 */
// /*01f0*/                   LDS.U.S8 R5, [R7+0x800] ;                              /* 0x0008000007057984 */
//                                                                                   /* 0x000f680000001200 */
// /*0200*/                   LDS.U.S8 R16, [R7+0x812] ;                             /* 0x0008120007107984 */
//                                                                                   /* 0x000f680000001200 */
// /*0210*/                   LDS.U.S8 R11, [R7+0x809] ;                             /* 0x00080900070b7984 */
//                                                                                   /* 0x000f680000001200 */
// /*0220*/                   LDS.U.S8 R12, [R7+0x811] ;                             /* 0x00081100070c7984 */
//                                                                                   /* 0x000f680000001200 */
// /*0230*/                   LDS.U.S8 R21, [R7+0x81b] ;                             /* 0x00081b0007157984 */
//                                                                                   /* 0x000f620000001200 */
// /*0240*/                   LOP3.LUT R23, R18, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff12177812 */
//                                                                                   /* 0x002fc600078ec0ff */
// /*0250*/                   LDS.U.64 R24, [R33.X8+0x400] ;                         /* 0x0004000021187984 */
//                                                                                   /* 0x000fe20000009a00 */
// /*0260*/                   PRMT R23, R23, 0x2104, R14 ;                           /* 0x0000210417177816 */
//                                                                                   /* 0x001fc6000000000e */
// /*0270*/                   LDSM.16.M88.4 R0, [R0+0xa00] ;                         /* 0x000a00000000783b */
//                                                                                   /* 0x000e220000000200 */
// /*0280*/                   LOP3.LUT R14, R19, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff130e7812 */
//                                                                                   /* 0x004fe400078ec0ff */
// /*0290*/                   PRMT R10, R23, 0x2104, R10 ;                           /* 0x00002104170a7816 */
//                                                                                   /* 0x008fca000000000a */
// /*02a0*/                   LDS.U.S8 R6, [R7+0x808] ;                              /* 0x0008080007067984 */
//                                                                                   /* 0x000e620000001200 */
// /*02b0*/                   PRMT R14, R14, 0x2104, R15 ;                           /* 0x000021040e0e7816 */
//                                                                                   /* 0x010fc6000000000f */
// /*02c0*/                   LDS.U.64 R26, [R33.X8+0x480] ;                         /* 0x00048000211a7984 */
//                                                                                   /* 0x000ea20000009a00 */
// /*02d0*/                   LOP3.LUT R15, R20, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff140f7812 */
//                                                                                   /* 0x020fc600078ec0ff */
// /*02e0*/                   LDS.U.S8 R8, [R7+0x810] ;                              /* 0x0008100007087984 */
//                                                                                   /* 0x000ee20000001200 */
// /*02f0*/                   PRMT R5, R10, 0x2104, R5 ;                             /* 0x000021040a057816 */
//                                                                                   /* 0x000fc60000000005 */
// /*0300*/                   LDS.U.S8 R17, [R7+0x81a] ;                             /* 0x00081a0007117984 */
//                                                                                   /* 0x000f220000001200 */
// /*0310*/                   PRMT R15, R15, 0x2104, R16 ;                           /* 0x000021040f0f7816 */
//                                                                                   /* 0x000fc60000000010 */
// /*0320*/                   LDS.U.S8 R13, [R7+0x819] ;                             /* 0x00081900070d7984 */
//                                                                                   /* 0x000f620000001200 */
// /*0330*/                   PRMT R11, R14, 0x2104, R11 ;                           /* 0x000021040e0b7816 */
//                                                                                   /* 0x000fc6000000000b */
// /*0340*/                   LDS.U.64 R28, [R33.X8+0x420] ;                         /* 0x00042000211c7984 */
//                                                                                   /* 0x000f620000009a00 */
// /*0350*/                   PRMT R15, R15, 0x2104, R12 ;                           /* 0x000021040f0f7816 */
//                                                                                   /* 0x000fc6000000000c */
// /*0360*/                   LDS.U.64 R30, [R33.X8+0x4a0] ;                         /* 0x0004a000211e7984 */
//                                                                                   /* 0x000f620000009a00 */
// /*0370*/                   LOP3.LUT R10, R21, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff150a7812 */
//                                                                                   /* 0x000fc600078ec0ff */
// /*0380*/                   LDS.U.S8 R9, [R7+0x818] ;                              /* 0x0008180007097984 */
//                                                                                   /* 0x000f620000001200 */
// /*0390*/                   IMMA.8816.S8.S8 R24, R0.ROW, R5.COL, R24 ;             /* 0x0000000500187237 */
//                                                                                   /* 0x001fe20000005418 */
// /*03a0*/                   PRMT R6, R11, 0x2104, R6 ;                             /* 0x000021040b067816 */
//                                                                                   /* 0x002fc60000000006 */
// /*03b0*/                   IMMA.8816.S8.S8 R26, R1.ROW, R5.COL, R26 ;             /* 0x00000005011a7237 */
//                                                                                   /* 0x004e22000000541a */
// /*03c0*/                   PRMT R8, R15, 0x2104, R8 ;                             /* 0x000021040f087816 */
//                                                                                   /* 0x008fe40000000008 */
// /*03d0*/                   PRMT R10, R10, 0x2104, R17 ;                           /* 0x000021040a0a7816 */
//                                                                                   /* 0x010fc80000000011 */
// /*03e0*/                   PRMT R10, R10, 0x2104, R13 ;                           /* 0x000021040a0a7816 */
//                                                                                   /* 0x020fc6000000000d */
// /*03f0*/                   IMMA.8816.S8.S8 R26, R3.ROW, R8.COL, R26 ;             /* 0x00000008031a7237 */
//                                                                                   /* 0x001fe8000000541a */
// /*0400*/                   IMMA.8816.S8.S8 R28, R0.ROW, R6.reuse.COL, R28 ;       /* 0x00000006001c7237 */
//                                                                                   /* 0x080e28000000541c */
// /*0410*/                   IMMA.8816.S8.S8 R30, R1.ROW, R6.COL, R30 ;             /* 0x00000006011e7237 */
//                                                                                   /* 0x000fe2000000541e */
// /*0420*/                   PRMT R9, R10, 0x2104, R9 ;                             /* 0x000021040a097816 */
//                                                                                   /* 0x000fc60000000009 */
// /*0430*/                   IMMA.8816.S8.S8 R0, R2.reuse.ROW, R8.COL, R24 ;        /* 0x0000000802007237 */
//                                                                                   /* 0x040e6a0000005418 */
// /*0440*/                   IMMA.8816.S8.S8 R28, R2.ROW, R9.COL, R28 ;             /* 0x00000009021c7237 */
//                                                                                   /* 0x0010a4000000541c */
// /*0450*/                   IADD3 R2, P0, R4, 0x4, RZ ;                            /* 0x0000000404027810 */
//                                                                                   /* 0x001fc60007f1e0ff */
// /*0460*/                   STS.64 [R33.X8], R0 ;                                  /* 0x0000000021007388 */
//                                                                                   /* 0x0021e20000008a00 */
// /*0470*/                   IMMA.8816.S8.S8 R30, R3.ROW, R9.COL, R30 ;             /* 0x00000009031e7237 */
//                                                                                   /* 0x000e62000000541e */
// /*0480*/                   ISETP.LE.U32.AND P0, PT, R2, c[0x0][0x68], !P0 ;       /* 0x00001a0002007a0c */
//                                                                                   /* 0x000fe20004703070 */
// /*0490*/                   IMAD.MOV.U32 R0, RZ, RZ, R26 ;                         /* 0x000000ffff007224 */
//                                                                                   /* 0x001fe400078e001a */
// /*04a0*/                   IMAD.MOV.U32 R1, RZ, RZ, R27 ;                         /* 0x000000ffff017224 */
//                                                                                   /* 0x000fd000078e001b */
// /*04b0*/                   STS.64 [R33.X8+0x80], R0 ;                             /* 0x0000800021007388 */
//                                                                                   /* 0x0005e40000008a00 */
// /*04c0*/                   IMAD.MOV.U32 R0, RZ, RZ, R28 ;                         /* 0x000000ffff007224 */
//                                                                                   /* 0x004fe400078e001c */
// /*04d0*/                   IMAD.MOV.U32 R1, RZ, RZ, R29 ;                         /* 0x000000ffff017224 */
//                                                                                   /* 0x000fd000078e001d */
// /*04e0*/                   STS.64 [R33.X8+0x20], R0 ;                             /* 0x0000200021007388 */
//                                                                                   /* 0x0003e40000008a00 */
// /*04f0*/                   IMAD.MOV.U32 R0, RZ, RZ, R30 ;                         /* 0x000000ffff007224 */
//                                                                                   /* 0x002fe400078e001e */
// /*0500*/                   IMAD.MOV.U32 R1, RZ, RZ, R31 ;                         /* 0x000000ffff017224 */
//                                                                                   /* 0x000fd000078e001f */
// /*0510*/                   STS.64 [R33.X8+0xa0], R0 ;                             /* 0x0000a00021007388 */


// m_size: 16, n_size: 8, k_size: 32, a_type: UINT8, b_type: UINT8, c_type: UINT32, result_type: UINT32
// RAW:
// /*0150*/                   LDS.U.S8 R12, [R7+0x403] ;                             /* 0x00040300070c7984 */
//                                                                                   /* 0x000e620000001200 */
// /*0160*/                   LOP3.LUT R0, R8.reuse, 0xf, RZ, 0xc0, !PT ;            /* 0x0000000f08007812 */
//                                                                                   /* 0x040fe200078ec0ff */
// /*0170*/                   IMAD R21, R5, 0x2, R6 ;                                /* 0x0000000205157824 */
//                                                                                   /* 0x000fe200078e0206 */
// /*0180*/                   LOP3.LUT R1, R8, 0x10, RZ, 0xc0, !PT ;                 /* 0x0000001008017812 */
//                                                                                   /* 0x001fe200078ec0ff */
// /*0190*/                   LDS.U.S8 R10, [R7+0x402] ;                             /* 0x00040200070a7984 */
//                                                                                   /* 0x000e280000001200 */
// /*01a0*/                   LDS.U.S8 R8, [R7+0x401] ;                              /* 0x0004010007087984 */
//                                                                                   /* 0x000ea20000001200 */
// /*01b0*/                   IMAD.IADD R0, R0, 0x1, R1 ;                            /* 0x0000000100007824 */
//                                                                                   /* 0x000fc600078e0201 */
// /*01c0*/                   LDS.U.S8 R5, [R7+0x400] ;                              /* 0x0004000007057984 */
//                                                                                   /* 0x000ee80000001200 */
// /*01d0*/                   LDS.U.S8 R13, [R7+0x413] ;                             /* 0x00041300070d7984 */
//                                                                                   /* 0x000f280000001200 */
// /*01e0*/                   LDS.U.S8 R11, [R7+0x412] ;                             /* 0x00041200070b7984 */
//                                                                                   /* 0x000f680000001200 */
// /*01f0*/                   LDS.U.S8 R9, [R7+0x411] ;                              /* 0x0004110007097984 */
//                                                                                   /* 0x000f680000001200 */
// /*0200*/                   LDS.U.64 R16, [R21.X8+0x200] ;                         /* 0x0002000015107984 */
//                                                                                   /* 0x000fe80000009a00 */
// /*0210*/                   LDSM.16.M88.4 R0, [R0+0x500] ;                         /* 0x000500000000783b */
//                                                                                   /* 0x000f700000000200 */
// /*0220*/                   LDS.U.64 R18, [R21.X8+0x280] ;                         /* 0x0002800015127984 */
//                                                                                   /* 0x000f620000009a00 */
// /*0230*/                   LOP3.LUT R15, R12, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff0c0f7812 */
//                                                                                   /* 0x002fc600078ec0ff */
// /*0240*/                   LDS.U.S8 R6, [R7+0x410] ;                              /* 0x0004100007067984 */
//                                                                                   /* 0x000e620000001200 */
// /*0250*/                   PRMT R15, R15, 0x2104, R10 ;                           /* 0x000021040f0f7816 */
//                                                                                   /* 0x001fc8000000000a */
// /*0260*/                   PRMT R8, R15, 0x2104, R8 ;                             /* 0x000021040f087816 */
//                                                                                   /* 0x004fc80000000008 */
// /*0270*/                   PRMT R5, R8, 0x2104, R5 ;                              /* 0x0000210408057816 */
//                                                                                   /* 0x008fe40000000005 */
// /*0280*/                   LOP3.LUT R8, R13, 0xff, RZ, 0xc0, !PT ;                /* 0x000000ff0d087812 */
//                                                                                   /* 0x010fc800078ec0ff */
// /*0290*/                   PRMT R8, R8, 0x2104, R11 ;                             /* 0x0000210408087816 */
//                                                                                   /* 0x020fc8000000000b */
// /*02a0*/                   PRMT R9, R8, 0x2104, R9 ;                              /* 0x0000210408097816 */
//                                                                                   /* 0x000fe20000000009 */
// /*02b0*/                   IMMA.8816.U8.U8 R16, R0.ROW, R5.reuse.COL, R16 ;       /* 0x0000000500107237 */
//                                                                                   /* 0x080e280000000410 */
// /*02c0*/                   IMMA.8816.U8.U8 R18, R1.ROW, R5.COL, R18 ;             /* 0x0000000501127237 */
//                                                                                   /* 0x0004e40000000412 */
// /*02d0*/                   IADD3 R5, P0, R4, 0x4, RZ ;                            /* 0x0000000404057810 */
//                                                                                   /* 0x004fe40007f1e0ff */
// /*02e0*/                   PRMT R6, R9, 0x2104, R6 ;                              /* 0x0000210409067816 */
//                                                                                   /* 0x002fe40000000006 */
// /*02f0*/                   ISETP.LE.U32.AND P0, PT, R5, c[0x0][0x68], !P0 ;       /* 0x00001a0005007a0c */
//                                                                                   /* 0x000fcc0004703070 */
// /*0300*/                   IMMA.8816.U8.U8 R0, R2.ROW, R6.reuse.COL, R16 ;        /* 0x0000000602007237 */
//                                                                                   /* 0x081e280000000410 */
// /*0310*/                   IMMA.8816.U8.U8 R2, R3.ROW, R6.COL, R18 ;              /* 0x0000000603027237 */
//                                                                                   /* 0x008e6c0000000412 */
// /*0320*/                   STS.64 [R21.X8], R0 ;                                  /* 0x0000000015007388 */
//                                                                                   /* 0x0011e80000008a00 */
// /*0330*/                   STS.64 [R21.X8+0x80], R2 ;                             /* 0x0000800215007388 */
//                                                                                   /* 0x0021e20000008a00 */


// m_size: 16, n_size: 8, k_size: 32, a_type: SINT8, b_type: SINT8, c_type: SINT32, result_type: SINT32
// RAW:
// /*0150*/                   LDS.U.S8 R12, [R7+0x403] ;                             /* 0x00040300070c7984 */
//                                                                                   /* 0x000e620000001200 */
// /*0160*/                   LOP3.LUT R0, R8.reuse, 0xf, RZ, 0xc0, !PT ;            /* 0x0000000f08007812 */
//                                                                                   /* 0x040fe200078ec0ff */
// /*0170*/                   IMAD R21, R5, 0x2, R6 ;                                /* 0x0000000205157824 */
//                                                                                   /* 0x000fe200078e0206 */
// /*0180*/                   LOP3.LUT R1, R8, 0x10, RZ, 0xc0, !PT ;                 /* 0x0000001008017812 */
//                                                                                   /* 0x001fe200078ec0ff */
// /*0190*/                   LDS.U.S8 R10, [R7+0x402] ;                             /* 0x00040200070a7984 */
//                                                                                   /* 0x000e280000001200 */
// /*01a0*/                   LDS.U.S8 R8, [R7+0x401] ;                              /* 0x0004010007087984 */
//                                                                                   /* 0x000ea20000001200 */
// /*01b0*/                   IMAD.IADD R0, R0, 0x1, R1 ;                            /* 0x0000000100007824 */
//                                                                                   /* 0x000fc600078e0201 */
// /*01c0*/                   LDS.U.S8 R5, [R7+0x400] ;                              /* 0x0004000007057984 */
//                                                                                   /* 0x000ee80000001200 */
// /*01d0*/                   LDS.U.S8 R13, [R7+0x413] ;                             /* 0x00041300070d7984 */
//                                                                                   /* 0x000f280000001200 */
// /*01e0*/                   LDS.U.S8 R11, [R7+0x412] ;                             /* 0x00041200070b7984 */
//                                                                                   /* 0x000f680000001200 */
// /*01f0*/                   LDS.U.S8 R9, [R7+0x411] ;                              /* 0x0004110007097984 */
//                                                                                   /* 0x000f680000001200 */
// /*0200*/                   LDS.U.64 R16, [R21.X8+0x200] ;                         /* 0x0002000015107984 */
//                                                                                   /* 0x000fe80000009a00 */
// /*0210*/                   LDSM.16.M88.4 R0, [R0+0x500] ;                         /* 0x000500000000783b */
//                                                                                   /* 0x000f700000000200 */
// /*0220*/                   LDS.U.64 R18, [R21.X8+0x280] ;                         /* 0x0002800015127984 */
//                                                                                   /* 0x000f620000009a00 */
// /*0230*/                   LOP3.LUT R15, R12, 0xff, RZ, 0xc0, !PT ;               /* 0x000000ff0c0f7812 */
//                                                                                   /* 0x002fc600078ec0ff */
// /*0240*/                   LDS.U.S8 R6, [R7+0x410] ;                              /* 0x0004100007067984 */
//                                                                                   /* 0x000e620000001200 */
// /*0250*/                   PRMT R15, R15, 0x2104, R10 ;                           /* 0x000021040f0f7816 */
//                                                                                   /* 0x001fc8000000000a */
// /*0260*/                   PRMT R8, R15, 0x2104, R8 ;                             /* 0x000021040f087816 */
//                                                                                   /* 0x004fc80000000008 */
// /*0270*/                   PRMT R5, R8, 0x2104, R5 ;                              /* 0x0000210408057816 */
//                                                                                   /* 0x008fe40000000005 */
// /*0280*/                   LOP3.LUT R8, R13, 0xff, RZ, 0xc0, !PT ;                /* 0x000000ff0d087812 */
//                                                                                   /* 0x010fc800078ec0ff */
// /*0290*/                   PRMT R8, R8, 0x2104, R11 ;                             /* 0x0000210408087816 */
//                                                                                   /* 0x020fc8000000000b */
// /*02a0*/                   PRMT R9, R8, 0x2104, R9 ;                              /* 0x0000210408097816 */
//                                                                                   /* 0x000fe20000000009 */
// /*02b0*/                   IMMA.8816.S8.S8 R16, R0.ROW, R5.reuse.COL, R16 ;       /* 0x0000000500107237 */
//                                                                                   /* 0x080e280000005410 */
// /*02c0*/                   IMMA.8816.S8.S8 R18, R1.ROW, R5.COL, R18 ;             /* 0x0000000501127237 */
//                                                                                   /* 0x0004e40000005412 */
// /*02d0*/                   IADD3 R5, P0, R4, 0x4, RZ ;                            /* 0x0000000404057810 */
//                                                                                   /* 0x004fe40007f1e0ff */
// /*02e0*/                   PRMT R6, R9, 0x2104, R6 ;                              /* 0x0000210409067816 */
//                                                                                   /* 0x002fe40000000006 */
// /*02f0*/                   ISETP.LE.U32.AND P0, PT, R5, c[0x0][0x68], !P0 ;       /* 0x00001a0005007a0c */
//                                                                                   /* 0x000fcc0004703070 */
// /*0300*/                   IMMA.8816.S8.S8 R0, R2.ROW, R6.reuse.COL, R16 ;        /* 0x0000000602007237 */
//                                                                                   /* 0x081e280000005410 */
// /*0310*/                   IMMA.8816.S8.S8 R2, R3.ROW, R6.COL, R18 ;              /* 0x0000000603027237 */
//                                                                                   /* 0x008e6c0000005412 */
// /*0320*/                   STS.64 [R21.X8], R0 ;                                  /* 0x0000000015007388 */
//                                                                                   /* 0x0011e80000008a00 */
// /*0330*/                   STS.64 [R21.X8+0x80], R2 ;                             /* 0x0000800215007388 */
//                                                                                   /* 0x0021e20000008a00 */
