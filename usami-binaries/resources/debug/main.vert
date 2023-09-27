#version 450

layout(location = 0) in highp vec4 a_position;

layout(location = 1) in highp vec2 a_texCoord;

layout(location = 0) out highp vec2 v_texCoord;

out gl_PerVertex { vec4 gl_Position; };

void main (void)
{
	gl_Position = a_position;
	v_texCoord = a_texCoord;
}