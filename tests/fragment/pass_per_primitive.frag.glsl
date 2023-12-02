#version 450
#extension GL_EXT_mesh_shader : require

layout(location = 0) in vec4 o_color;
layout(location = 0) perprimitiveEXT out vec4 uFragColor;

void main(void)
{
    uFragColor = o_color;
}