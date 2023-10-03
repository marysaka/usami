#version 450

layout (triangles) in;
layout (line_strip, max_vertices = 6) out;

layout (binding = 1) uniform UBO 
{
	mat4 projection;
	mat4 model;
} ubo;

layout (location = 0) in vec3 inNormal[];

layout (location = 0) out vec3 outColor;

void main(void)
{	
	gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
	outColor = vec3(1.0, 0.0, 0.0);
	EmitVertex();

	gl_Position = vec4(1.0, 1.0, 0.0, 1.0);
	outColor = vec3(0.0, 0.0, 1.0);
	EmitVertex();
	EndPrimitive();
	
	gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
	outColor = vec3(1.0, 0.0, 0.0);
	EmitVertex();

	gl_Position = vec4(-1.0, -1.0, 0.0, 1.0);
	outColor = vec3(1.0, 1.0, 1.0);
	EmitVertex();
	EndPrimitive();
}