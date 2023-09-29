#version 450
#extension GL_NV_shader_sm_builtins : enable

layout(location = 0) out vec3 frag_pos;

void main(void)
{
    gl_Position = vec4(gl_WarpIDNV, 1.0f, 1.0f, 1.0f);
    frag_pos = gl_Position.xyz;
}