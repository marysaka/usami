#version 450

#extension GL_EXT_mesh_shader : require

const uint workGroupSize = 4;
const uint taskInvocationCount = 1;
const uint meshInvocationCount = 1;
layout (local_size_x=taskInvocationCount, local_size_y=1, local_size_z=1) in;

struct TaskData {
    uint value;
};

layout(binding = 0) buffer UniformBufferObject {
    uint taskInvocations[taskInvocationCount * workGroupSize];
} result;

taskPayloadSharedEXT TaskData td;

void main() 
{
    const uint workGroupIndex = gl_NumWorkGroups.x * gl_NumWorkGroups.y * gl_WorkGroupID.z + gl_NumWorkGroups.x * gl_WorkGroupID.y + gl_WorkGroupID.x;
    td.value = workGroupIndex;

	EmitMeshTasksEXT(4, 1, 1);
}