#version 440

precision highp iimage2D;

layout (local_size_x = 8, local_size_y = 8, local_size_z = 1) in;
layout (r32i, binding=0) volatile uniform iimage2D u_image;
void main (void)
{
	int gx = int(gl_GlobalInvocationID.x);
	int gy = int(gl_GlobalInvocationID.y);
	int gz = int(gl_GlobalInvocationID.z);
	imageStore(u_image, ivec2(gx,gy), ivec4(gx^gy^gz));

	memoryBarrier();
	barrier();

	int sum = int(0);
	int groupBaseX = gx/8*8;
	int groupBaseY = gy/8*8;
	int groupBaseZ = gz/1*1;
	int xOffsets[] = int[]( 1, 4, 7, 10 );
	int yOffsets[] = int[]( 2, 5, 8, 11 );
	int zOffsets[] = int[]( 3, 6, 9, 12 );
	for (int i = 0; i < 4; i++)
	{
		int readX = groupBaseX + (gx + xOffsets[i]) % 8;
		int readY = groupBaseY + (gy + yOffsets[i]) % 8;
		int readZ = groupBaseZ + (gz + zOffsets[i]) % 1;
		sum += imageLoad(u_image, ivec2(readX,readY)).x;
	}


	imageStore(u_image, ivec2(gx,gy), ivec4(sum));
}
