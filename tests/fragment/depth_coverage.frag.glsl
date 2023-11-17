#version 450

layout(early_fragment_tests) in;
layout(location = 0) in vec4 vtxColor;
layout(location = 0) out vec4 fragColor;

void main (void)
{
    const int coveredSamples = bitCount(gl_SampleMaskIn[0]);
    fragColor = vtxColor * (1.0 / 2 * coveredSamples);
}