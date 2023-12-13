#version 450
#extension GL_EXT_mesh_shader : enable
#extension GL_EXT_shader_8bit_storage: require

layout(binding = 0) buffer UniformBufferObject {
    uint data[0x1000];
} ubo;


layout (location=0) perprimitiveEXT in vec4 primitiveColor;
layout (location=0) out vec4 outColor;

void main ()
{
    outColor = primitiveColor;
}