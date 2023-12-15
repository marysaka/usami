

#version 450
#extension GL_EXT_mesh_shader : require
#extension GL_KHR_shader_subgroup_vote : require

const uint invocationCount = 32;

layout (local_size_x=invocationCount, local_size_y=1, local_size_z=1) in;
layout (triangles) out;
layout (max_vertices=128, max_primitives=128) out;

layout(binding = 0) buffer UniformBufferObject {
    //uint temporal_result[invocationCount];
    uint sharedOutput;
} result;

shared uint sharedElement;

void main (void) {
    uint atomicValue = atomicAdd(sharedElement, 1);

    if (gl_LocalInvocationIndex == 0) {
        result.sharedOutput = atomicAdd(sharedElement, 1);
    }

    //result.temporal_result[gl_LocalInvocationIndex] = atomicValue;

    SetMeshOutputsEXT(0u, 0u);
}