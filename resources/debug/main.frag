#version 450

layout(location = 0) in highp vec2 v_texCoord;
layout(location = 0) out mediump vec4 u_fragColor;

layout (set = 0, binding = 0, std140) uniform Block 
{
    highp float u_bias;
    highp float u_ref;
    highp vec4 u_colorScale;
    highp vec4 u_colorBias;
};

layout(set = 1, binding = 0) uniform highp sampler2D u_sampler;

void main(void)
{
    highp vec2 texCoord = v_texCoord.yx;
    u_fragColor = texture(u_sampler, texCoord) * u_colorScale + u_colorBias;
}
