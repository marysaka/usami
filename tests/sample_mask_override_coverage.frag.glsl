#version 450
#extension GL_NV_sample_mask_override_coverage : enable

layout(location = 0) in vec4 color;

layout(location = 0, override_coverage) out int gl_SampleMask[];

void main() {
    gl_SampleMask[0] = int(0xFFFFFFFF);
}