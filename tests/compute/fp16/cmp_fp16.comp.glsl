#version 450

#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_EXT_shader_16bit_storage : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;


//        /*0050*/                   IMAD.U32 R0, RZ, RZ, UR6 ;                                  /* 0x00000006ff007e24 */
//                                                                                               /* 0x000fcc000f8e00ff */
//        /*0060*/                   HSETP2.GT.AND P0, PT, R0.H0_H0, cx[UR4] [0x10].H0_H0, PT ;  /* 0x2000040400007634 */
//                                                                                               /* 0x000fe2000bf04800 */
//        /*0070*/                   ULDC.64 UR4, c[0x0][0x40] ;                                 /* 0x0000100000047ab9 */
//                                                                                               /* 0x000fce0000000a00 */
//        /*0080*/                   SEL R0, RZ, 0x1, !P0 ;                                      /* 0x00000001ff007807 */
//                                                                                               /* 0x000fd00004000000 */
//        /*0090*/                   STG.E.STRONG.CTA [UR4], R0 ;                                /* 0x00000000ff007986 */
//                                                                                               /* 0x000fe2000c110904 */
#define input_float float16_t


layout(set = 0, binding = 0) buffer OutBuf { bool result; }
out_buf;

layout(set = 0, binding = 1) uniform InBuf { input_float inputs[2]; }
in_buf;

// See https://docs.nvidia.com/cuda/cuda-math-api
void main(void) {
  out_buf.result = in_buf.inputs[0] > in_buf.inputs[1];
}
