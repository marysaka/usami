#version 450

#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_EXT_shader_16bit_storage : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

//        /*0050*/                   MOV R0, UR7 ;                                         /* 0x0000000700007c02 */
//                                                                                         /* 0x000fe20008000f00 */
//        /*0060*/                   IMAD.U32 R1, RZ, RZ, UR6 ;                            /* 0x00000006ff017e24 */
//                                                                                         /* 0x000fcc000f8e00ff */
//        /*0070*/                   HFMA2 R0, R1.H0_H0, R0.H0_H0, cx[UR4] [0x20].H0_H0 ;  /* 0x2000080401007631 */
//                                                                                         /* 0x000fe20008040800 */
//        /*0080*/              @!P0 EXIT ;                                                /* 0x000000000000894d */
//                                                                                         /* 0x000fee0003800000 */
//        /*0090*/                   ULDC.64 UR4, c[0x0][0x40] ;                           /* 0x0000100000047ab9 */
//                                                                                         /* 0x000fe40000000a00 */
//        /*00a0*/                   STG.E.U16.STRONG.CTA [UR4], R0 ;                      /* 0x00000000ff007986 */
//                                                                                         /* 0x000fe2000c110504 */
#define input_float float16_t
#define output_float float16_t

//        /*00b0*/                   MOV R1, UR8 ;                            /* 0x0000000800017c02 */
//                                                                            /* 0x000fc40008000f00 */
//        /*00c0*/                   MOV R3, UR5 ;                            /* 0x0000000500037c02 */
//                                                                            /* 0x000fe40008000f00 */
//        /*00d0*/                   PRMT R0, R0, 0x5410, R1 ;                /* 0x0000541000007816 */
//                                                                            /* 0x000fe40000000001 */
//        /*00e0*/                   PRMT R1, R2, 0x5410, R3 ;                /* 0x0000541002017816 */
//                                                                            /* 0x000fcc0000000003 */
//        /*00f0*/                   HFMA2 R0, R0, R1, cx[UR12] [0x20] ;      /* 0x0000080c00007631 */
//                                                                            /* 0x000fe20008000001 */
//        /*0100*/              @!P0 EXIT ;                                   /* 0x000000000000894d */
//                                                                            /* 0x000fee0003800000 */
//        /*0110*/                   ULDC.64 UR4, c[0x0][0x40] ;              /* 0x0000100000047ab9 */
//                                                                            /* 0x000fe40000000a00 */
//        /*0120*/                   STG.E.STRONG.CTA [UR4], R0 ;             /* 0x00000000ff007986 */
//                                                                            /* 0x000fe2000c110904 */
// #define input_float f16vec2
// #define output_float f16vec2

//        /*0050*/                   MOV R0, UR7 ;                                             /* 0x0000000700007c02 */
//                                                                                             /* 0x000fe20008000f00 */
//        /*0060*/                   IMAD.U32 R1, RZ, RZ, UR6 ;                                /* 0x00000006ff017e24 */
//                                                                                             /* 0x000fcc000f8e00ff */
//        /*0070*/                   HFMA2.F32 R0, R1.H0_H0, R0.H0_H0, cx[UR4] [0x20].H0_H0 ;  /* 0x2000080401007631 */
//                                                                                             /* 0x000fe20008044800 */
//        /*0080*/              @!P0 EXIT ;                                                    /* 0x000000000000894d */
//                                                                                             /* 0x000fee0003800000 */
//        /*0090*/                   ULDC.64 UR4, c[0x0][0x40] ;                               /* 0x0000100000047ab9 */
//                                                                                             /* 0x000fe40000000a00 */
//        /*00a0*/                   STG.E.STRONG.CTA [UR4], R0 ;                              /* 0x00000000ff007986 */
//                                                                                             /* 0x000fe2000c110904 */
// #define input_float float16_t
// #define output_float float


//        /*0070*/                   MOV R0, UR7 ;                                             /* 0x0000000700007c02 */
//                                                                                             /* 0x000fe20008000f00 */
//        /*0080*/                   ULDC.U16 UR9, cx[UR4][0x12] ;                             /* 0x0000048004097ab9 */
//                                                                                             /* 0x000fe20008000400 */
//        /*0090*/                   IMAD.U32 R3, RZ, RZ, UR8 ;                                /* 0x00000008ff037e24 */
//                                                                                             /* 0x000fe2000f8e00ff */
//        /*00a0*/                   MOV R2, UR9 ;                                             /* 0x0000000900027c02 */
//                                                                                             /* 0x000fc60008000f00 */
//        /*00b0*/                   HFMA2.F32 R0, R1.H0_H0, R0.H0_H0, cx[UR4] [0x20].H0_H0 ;  /* 0x2000080401007631 */
//                                                                                             /* 0x000fc60008044800 */
//        /*00c0*/                   HFMA2.F32 R1, R3.H0_H0, R2.H0_H0, cx[UR4] [0x20].H1_H1 ;  /* 0x3000080403017631 */
//                                                                                             /* 0x000fe20008044802 */
//        /*00d0*/              @!P0 EXIT ;                                                    /* 0x000000000000894d */
//                                                                                             /* 0x000fee0003800000 */
//        /*00e0*/                   ULDC.64 UR4, c[0x0][0x40] ;                               /* 0x0000100000047ab9 */
//                                                                                             /* 0x000fe40000000a00 */
//        /*00f0*/                   STG.E.64.STRONG.CTA [UR4], R0 ;                           /* 0x00000000ff007986 */
//                                                                                             /* 0x000fe2000c110b04 */
// #define input_float f16vec2
// #define output_float vec2

layout(set = 0, binding = 0) buffer OutBuf { output_float result; }
out_buf;

layout(set = 0, binding = 1) uniform InBuf { input_float inputs[3]; }
in_buf;

// See https://docs.nvidia.com/cuda/cuda-math-api
void main(void) {
  out_buf.result = in_buf.inputs[0] * in_buf.inputs[1] + in_buf.inputs[2];
}
