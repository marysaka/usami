#version 450
#extension GL_EXT_fragment_shader_barycentric : enable

layout(location = 0) in float o_u;
layout(location = 1) in float o_v;
layout(location = 0) out vec4 u_fragColor;

layout(binding = 0) uniform sampler2D u_sampler;

void main(void)
{
    u_fragColor = texture(u_sampler, vec2(o_u * gl_BaryCoordNoPerspEXT.x, o_v));
}
