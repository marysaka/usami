#version 450

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(r32f, binding = 2) uniform image1DArray u_image[2];

void main(void) { imageStore(u_image[1], ivec2(1, 0), vec4(1.0)); }
