#version 450

layout(location = 0) in vec3 frag_pos;
layout(location = 0) out vec4 uFragColor;

void main(void)
{
    float dX = dFdxCoarse(frag_pos.x);

    uFragColor = vec4(dX);
}
