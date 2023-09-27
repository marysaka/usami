#version 450

layout (location = 0) in vec4 pos;
layout (location = 1) in vec2 uv;

layout (location = 0) out vec2 o_uv;

void main(void)
{
    o_uv = uv;
    gl_Position = pos;
}
