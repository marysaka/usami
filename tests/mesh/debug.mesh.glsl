#version 450
#extension GL_EXT_mesh_shader : enable

layout (local_size_x=1) in;
layout (max_primitives=2, max_vertices=4) out;
layout (triangles) out;

out gl_MeshPerVertexEXT {
    vec4  gl_Position;
    float gl_PointSize;
    float gl_ClipDistance[1];
} gl_MeshVerticesEXT[];

layout (location=0) out vec4 customAttribute1[];
layout (location=1) out flat float customAttribute2[];
layout (location=2) out int customAttribute3[];

layout (location=3) out perprimitiveEXT uvec4 customAttribute4[];
layout (location=4) out perprimitiveEXT float customAttribute5[];

out perprimitiveEXT gl_MeshPerPrimitiveEXT {
  int gl_PrimitiveID;
  int gl_ViewportIndex;
} gl_MeshPrimitivesEXT[];

void main ()
{
    SetMeshOutputsEXT(4u, 2u);

    gl_MeshVerticesEXT[0].gl_Position = vec4(-1.0, -1.0, 0.0, 1.0);
    gl_MeshVerticesEXT[1].gl_Position = vec4( 1.0, -1.0, 0.0, 1.0);
    gl_MeshVerticesEXT[2].gl_Position = vec4(-1.0,  1.0, 0.0, 1.0);
    gl_MeshVerticesEXT[3].gl_Position = vec4( 1.0,  1.0, 0.0, 1.0);

    gl_MeshVerticesEXT[0].gl_PointSize = 1.0;
    gl_MeshVerticesEXT[1].gl_PointSize = 1.0;
    gl_MeshVerticesEXT[2].gl_PointSize = 1.0;
    gl_MeshVerticesEXT[3].gl_PointSize = 1.0;

    // Remove geometry on the right side.
    gl_MeshVerticesEXT[0].gl_ClipDistance[0] =  1.0;
    gl_MeshVerticesEXT[1].gl_ClipDistance[0] = -1.0;
    gl_MeshVerticesEXT[2].gl_ClipDistance[0] =  1.0;
    gl_MeshVerticesEXT[3].gl_ClipDistance[0] = -1.0;
    
    gl_PrimitiveTriangleIndicesEXT[0] = uvec3(0, 1, 2);
    gl_PrimitiveTriangleIndicesEXT[1] = uvec3(2, 3, 1);

    gl_MeshPrimitivesEXT[0].gl_PrimitiveID = 1000;
    gl_MeshPrimitivesEXT[1].gl_PrimitiveID = 1001;

    gl_MeshPrimitivesEXT[0].gl_ViewportIndex = 1;
    gl_MeshPrimitivesEXT[1].gl_ViewportIndex = 1;

    // Custom per-vertex attributes
    customAttribute1[0] = vec4(0.25, 0.5, 10.0, 3.0);
    customAttribute1[1] = vec4(0.25, 1.0, 20.0, 3.0);
    customAttribute1[2] = vec4( 0.5, 0.5, 20.0, 3.0);
    customAttribute1[3] = vec4( 0.5, 1.0, 10.0, 3.0);

    customAttribute2[0] = 1.0f;
    customAttribute2[1] = 1.0f;
    customAttribute2[2] = 2.0f;
    customAttribute2[3] = 2.0f;

    customAttribute3[0] = 3;
    customAttribute3[1] = 3;
    customAttribute3[2] = 4;
    customAttribute3[3] = 4;

    // Custom per-primitive attributes.
    customAttribute4[0] = uvec4(100, 101, 102, 103);
    customAttribute4[1] = uvec4(200, 201, 202, 203);

    customAttribute5[0] = 6.0;
    customAttribute5[1] = 7.0;
}