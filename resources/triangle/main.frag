#version 450

layout (location = 0) in vec4 o_color;
layout (location = 0) out vec4 uFragColor;

void main(void)
{
    uFragColor = o_color;
}