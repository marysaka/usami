#version 450

layout(location = 0) in vec2 o_uv;
layout(location = 0) out vec4 u_fragColor;

layout(binding = 0) uniform sampler2D u_sampler;

void main(void)
{
    u_fragColor = texture(u_sampler, o_uv);
}
