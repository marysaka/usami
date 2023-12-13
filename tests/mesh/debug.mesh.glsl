#version 450
#extension GL_EXT_mesh_shader : enable

layout (local_size_x=128, local_size_y=1, local_size_z=1) in;
layout (triangles) out;
layout (max_vertices=3, max_primitives=1) out;
const uint payloadElements = 1u;

// 28672
const uint sharedMemoryElements = 28672 / 4;

layout(binding = 0) buffer UniformBufferObject {
    uint sharedOK;
    uint sharedMemoryErrorIndex;
    uint sharedMemoryErrorValue;
    uint sharedMemoryErrorExpected;
    uint localInvocationCount[128];
} result;

shared uint sharedElements[sharedMemoryElements];

void main () {
    result.localInvocationCount[gl_LocalInvocationIndex] = gl_LocalInvocationIndex + 1;

    const uint shMemElementsPerInvocation = uint(ceil(float(sharedMemoryElements) / float(128)));
    for (uint i = 0u; i < shMemElementsPerInvocation; ++i) {
        const uint elemIdx = shMemElementsPerInvocation * gl_LocalInvocationIndex + i;
        if (elemIdx < sharedMemoryElements) {
            sharedElements[elemIdx] = elemIdx * 2u + 1000u;
        }
    }
    memoryBarrierShared();
    barrier();
    for (uint i = 0u; i < shMemElementsPerInvocation; ++i) {
        const uint elemIdx = shMemElementsPerInvocation * gl_LocalInvocationIndex + i;
        if (elemIdx < sharedMemoryElements) {
            const uint accessIdx = sharedMemoryElements - 1u - elemIdx;
            sharedElements[accessIdx] += accessIdx;
        }
    }
    memoryBarrierShared();
    barrier();
    if (gl_LocalInvocationIndex == 0u) {
        bool allOK = true;
        for (uint i = 0u; i < sharedMemoryElements; ++i) {
            if (sharedElements[i] != i*3u + 1000u) {
                allOK = false;
                result.sharedMemoryErrorIndex = i;
                result.sharedMemoryErrorValue = sharedElements[i];
                result.sharedMemoryErrorExpected =  i*3u + 1000u;
                break;
            }
        }
        result.sharedOK = (allOK ? 1u : 0u);
    }

    SetMeshOutputsEXT(0u, 0u);
}