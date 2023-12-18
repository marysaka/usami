#version 450
#extension GL_EXT_mesh_shader : enable

const uint workGroupSize = 4;
const uint taskInvocationCount = 1 * workGroupSize;
const uint meshInvocationCount = 1 * workGroupSize;

layout (local_size_x=meshInvocationCount, local_size_y=1, local_size_z=1) in;
layout (triangles) out;
layout (max_vertices=3, max_primitives=1) out;
const uint payloadElements = 1u;

layout(binding = 0) buffer UniformBufferObject {
    uint taskInvocations[taskInvocationCount];
    uint meshInvocations[meshInvocationCount];
} result;

void main () {
    const uint workGroupIndex = gl_NumWorkGroups.x * gl_NumWorkGroups.y * gl_WorkGroupID.z + gl_NumWorkGroups.x * gl_WorkGroupID.y + gl_WorkGroupID.x;
    result.meshInvocations[gl_LocalInvocationIndex * workGroupIndex] = workGroupIndex + 1;

    SetMeshOutputsEXT(0u, 0u);
}