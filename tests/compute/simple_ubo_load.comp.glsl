#version 450

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer OutBuf { uint outputValue; }
out_buf;

layout(set = 0, binding = 1) uniform InBuf { uint value; }
in_buf;

void main(void) { out_buf.outputValue = in_buf.value; }
