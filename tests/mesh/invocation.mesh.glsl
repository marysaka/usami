#version 450
#extension GL_EXT_mesh_shader : enable

const uint threadsCount = 32;

const uint meshInvocationCount = threadsCount * 2;

layout (local_size_x=meshInvocationCount, local_size_y=1, local_size_z=1) in;
layout (points) out;
layout (max_vertices=1, max_primitives=1) out;

layout(binding = 0) buffer UniformBufferObject {
    uvec4 meshData[meshInvocationCount];
} result;

void main () {
    SetMeshOutputsEXT(0u, 0u);

    result.meshData[gl_LocalInvocationIndex].x = 0x10000 + gl_LocalInvocationIndex << 0;

    uint target_index = gl_LocalInvocationIndex >= threadsCount ? gl_LocalInvocationIndex - threadsCount : gl_LocalInvocationIndex + threadsCount;

    result.meshData[target_index].x += 0x100000 + gl_LocalInvocationIndex << 8;
}