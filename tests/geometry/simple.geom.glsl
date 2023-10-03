#version 450
#extension GL_NV_geometry_shader_passthrough : require


layout (triangles) in;
layout (line_strip, max_vertices = 6) out;

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
}