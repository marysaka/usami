#version 450

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout (r32f, binding=0) uniform image1D u_image;

void main (void)
{
	int gx = int(gl_GlobalInvocationID.x);
	int gy = int(gl_GlobalInvocationID.y);
	int gz = int(gl_GlobalInvocationID.z);

	int groupBaseX = gx/8*8;

	for (int i = 0; i < 4; i++) {
		imageStore(u_image, i, vec4((i ^ groupBaseX)));
	}
}
