#version 450

#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_EXT_shader_16bit_storage : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;


//        /*0050*/                   MOV R0, UR6 ;                               /* 0x0000000600007c02 */
//                                                                               /* 0x000fcc0008000f00 */
//        /*0060*/                   HMUL2 R0, R0.H0_H0, cx[UR4] [0x10].H0_H0 ;  /* 0x2000040400007a32 */
//                                                                               /* 0x000fe20008000800 */
//        /*0070*/                   ULDC.64 UR4, c[0x0][0x40] ;                 /* 0x0000100000047ab9 */
//                                                                               /* 0x000fce0000000a00 */
//        /*0080*/                   STG.E.U16.STRONG.CTA [UR4], R0 ;            /* 0x00000000ff007986 */
//                                                                               /* 0x000fe2000c110504 */
// #define input_float float16_t
// #define output_float float16_t

//        /*0080*/                   MOV R0, UR6 ;                            /* 0x0000000600007c02 */
//                                                                            /* 0x000fe20008000f00 */
//        /*0090*/                   IMAD.U32 R1, RZ, RZ, UR5 ;               /* 0x00000005ff017e24 */
//                                                                            /* 0x000fe2000f8e00ff */
//        /*00a0*/                   ULDC.64 UR4, c[0x0][0x40] ;              /* 0x0000100000047ab9 */
//                                                                            /* 0x000fc80000000a00 */
//        /*00b0*/                   PRMT R0, R0, 0x5410, R1 ;                /* 0x0000541000007816 */
//                                                                            /* 0x000fcc0000000001 */
//        /*00c0*/                   HMUL2 R0, R0, cx[UR8] [0x10] ;           /* 0x0000040800007a32 */
//                                                                            /* 0x000fd00008000000 */
//        /*00d0*/                   STG.E.STRONG.CTA [UR4], R0 ;             /* 0x00000000ff007986 */
//                                                                            /* 0x000fe2000c110904 */
// #define input_float f16vec2
// #define output_float f16vec2

//        /*0050*/                   MOV R0, UR6 ;                                   /* 0x0000000600007c02 */
//                                                                                   /* 0x000fcc0008000f00 */
//        /*0060*/                   HMUL2.F32 R0, R0.H0_H0, cx[UR4] [0x10].H0_H0 ;  /* 0x2000040400007a32 */
//                                                                                   /* 0x000fe20008004800 */
//        /*0070*/                   ULDC.64 UR4, c[0x0][0x40] ;                     /* 0x0000100000047ab9 */
//                                                                                   /* 0x000fce0000000a00 */
//        /*0080*/                   STG.E.STRONG.CTA [UR4], R0 ;                    /* 0x00000000ff007986 */
//                                                                                   /* 0x000fe2000c110904 */
// #define input_float float16_t
// #define output_float float

//        /*0060*/                   MOV R0, UR6 ;                                   /* 0x0000000600007c02 */
//                                                                                   /* 0x000fe20008000f00 */
//        /*0070*/                   IMAD.U32 R1, RZ, RZ, UR7 ;                      /* 0x00000007ff017e24 */
//                                                                                   /* 0x000fca000f8e00ff */
//        /*0080*/                   HMUL2.F32 R0, R0.H0_H0, cx[UR4] [0x10].H0_H0 ;  /* 0x2000040400007a32 */
//                                                                                   /* 0x000fe40008004800 */
//        /*0090*/                   HMUL2.F32 R1, R1.H0_H0, cx[UR4] [0x10].H1_H1 ;  /* 0x3000040401017a32 */
//                                                                                   /* 0x000fe20008004800 */
//        /*00a0*/                   ULDC.64 UR4, c[0x0][0x40] ;                     /* 0x0000100000047ab9 */
//                                                                                   /* 0x000fce0000000a00 */
//        /*00b0*/                   STG.E.64.STRONG.CTA [UR4], R0 ;                 /* 0x00000000ff007986 */
//                                                                                   /* 0x000fe2000c110b04 */
// #define input_float f16vec2
// #define output_float vec2

#define input_float f16mat4x4
#define output_float f16mat4x4

layout(set = 0, binding = 0) buffer OutBuf { output_float result; }
out_buf;

layout(set = 0, binding = 1) uniform InBuf { input_float inputs[2]; }
in_buf;

// See https://docs.nvidia.com/cuda/cuda-math-api
void main(void) {
  out_buf.result = in_buf.inputs[0] * in_buf.inputs[1];
}
