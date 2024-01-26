#version 450

#define IMAGE_SIZE_STORAGE_image1D int
#define IMAGE_SIZE_STORAGE_image2D ivec2
#define IMAGE_SIZE_STORAGE_image3D ivec3
#define IMAGE_SIZE_STORAGE_imageCube ivec2
#define IMAGE_SIZE_STORAGE_imageCubeArray ivec3
#define IMAGE_SIZE_STORAGE_image1DArray ivec2
#define IMAGE_SIZE_STORAGE_image2DArray ivec3
#define IMAGE_SIZE_STORAGE_image2DMS ivec2
#define IMAGE_SIZE_STORAGE_image2DMSArray ivec3

#define IMAGE_TYPE image1D
#define IMAGE_SIZE_STORAGE IMAGE_SIZE_STORAGE_image1D

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(binding = 0) buffer OutBuf { IMAGE_SIZE_STORAGE outputValue; }
out_buf;

layout(r32f, binding = 2) uniform IMAGE_TYPE u_image;

void main(void) { out_buf.outputValue = imageSize(u_image); }
