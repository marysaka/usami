#version 450

#extension GL_EXT_mesh_shader : require

const uint workGroupSize = 4;
const uint taskInvocationCount = 1 * workGroupSize;
const uint meshInvocationCount = 1 * workGroupSize;

layout(binding = 0) buffer UniformBufferObject {
    uint taskInvocations[taskInvocationCount];
    uint meshInvocations[meshInvocationCount];
} result;

void main() 
{
    const uint workGroupIndex = gl_NumWorkGroups.x * gl_NumWorkGroups.y * gl_WorkGroupID.z + gl_NumWorkGroups.x * gl_WorkGroupID.y + gl_WorkGroupID.x;
    result.taskInvocations[gl_LocalInvocationIndex * workGroupIndex] = workGroupIndex;

	EmitMeshTasksEXT(1, 1, 1);
}