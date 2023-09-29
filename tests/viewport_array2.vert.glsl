#version 450


#extension GL_ARB_shader_viewport_layer_array : require
#extension GL_NV_viewport_array2 : require

layout(location = 0) in vec4 pos;
layout(location = 1) in vec4 color;

layout(location = 0) out vec4 o_color;
layout(location = 1) out vec3 frag_pos;

layout (viewport_relative) out highp int gl_Layer;


void main(void)
{
    o_color = color;
    gl_Position = pos;
    frag_pos = gl_Position.xyz;

    gl_ViewportMask[0] = 1;
}