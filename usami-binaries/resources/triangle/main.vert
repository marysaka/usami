#version 450

layout(location = 0) in vec4 pos;
layout(location = 1) in vec4 color;

layout(location = 0) out vec4 o_color;
layout(location = 1) out vec3 frag_pos;

void main(void)
{
    o_color = color;
    frag_pos = gl_Position.xyz;
}