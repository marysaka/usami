

#version 450
#extension GL_EXT_mesh_shader : enable

layout (local_size_x=1, local_size_y=1, local_size_z=1) in;
layout (triangles) out;
layout (max_vertices=128, max_primitives=128) out;

out gl_MeshPerVertexEXT {
    vec4  gl_Position;
} gl_MeshVerticesEXT[];

const uint maxLocations = 31u;
struct LocationStruct {
    uvec4 location_var[maxLocations];
};
layout (location=0) perprimitiveEXT flat out LocationStruct ls[];

void main (void) {
    SetMeshOutputsEXT(128u, 128u);

    for (uint i = 0u; i < maxLocations; ++i) {
        const uint baseVal = 10000u * (i + 1u);
        const uvec4 expectedValue = uvec4(baseVal + 1u, baseVal + 2u, baseVal + 3u, baseVal + 4u);
        gl_MeshVerticesEXT[(i + 0) * 2].gl_Position = vec4( 0.0, -0.5, 0.0, 1.0);
        gl_MeshVerticesEXT[(i + 1) * 2].gl_Position = vec4(-0.5,  0.5, 0.0, 1.0);
        gl_MeshVerticesEXT[(i + 2) * 2].gl_Position = vec4( 0.5,  0.5, 0.0, 1.0);
        gl_PrimitiveTriangleIndicesEXT[i] = uvec3(i + 0, i + 1, i + 2);

        ls[0].location_var[i] = expectedValue;
    }
}