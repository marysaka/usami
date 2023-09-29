#version 450
layout(location = 0) in vec4 color;

void main() {
    gl_SampleMask[0] = int(0xFFFFFFFF);
}