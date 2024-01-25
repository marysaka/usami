#version 450

#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_EXT_shader_16bit_storage : require
#extension GL_EXT_shader_atomic_float2 : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

#define input_float float16_t
#define output_float float16_t

layout(set = 0, binding = 0) buffer OutBuf { output_float result; }
out_buf;

layout(set = 0, binding = 1) uniform InBuf { input_float inputs[2]; }
in_buf;

void main(void) {
  out_buf.result = atomicAdd(out_buf.result, in_buf.inputs[0]);
}
