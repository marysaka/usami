#version 450

#extension GL_NV_shading_rate_image : require

layout(location = 0) in vec3 frag_pos;
layout(location = 0) out vec4 uFragColor;

void main(void)
{
    uFragColor = vec4(gl_InvocationsPerPixelNV.x);
}
