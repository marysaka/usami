#version 450
#extension GL_ARB_post_depth_coverage : require

layout(early_fragment_tests) in;
layout(post_depth_coverage) in;
layout(location = 0) in vec4 vtxColor;
layout(location = 0) out vec4 fragColor;

void main (void)
{
    const int coveredSamples = bitCount(gl_SampleMaskIn[0]);
    fragColor = vtxColor * (1.0 / 2 * coveredSamples);
}