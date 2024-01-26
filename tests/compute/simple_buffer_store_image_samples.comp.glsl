#version 450

#define IMAGE_TYPE image2DMS
// #define IMAGE_TYPE image2DMSArray

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(binding = 1) buffer OutBuf {
  int outputValue;
  int outputValue2;
}
out_buf;

layout(r32f, binding = 2) uniform IMAGE_TYPE u_image;

void main(void) {
  out_buf.outputValue2 = imageSamples(u_image) * out_buf.outputValue;
}
