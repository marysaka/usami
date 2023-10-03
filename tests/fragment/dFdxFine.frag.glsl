#version 450

layout(location = 0) in vec3 frag_pos;
layout(location = 0) out vec4 uFragColor;

void main(void)
{
    float dX = dFdxFine(frag_pos.x);

    uFragColor = vec4(dX);
}
