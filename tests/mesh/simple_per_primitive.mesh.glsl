#version 450
#extension GL_EXT_mesh_shader : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(triangles, max_vertices = 3, max_primitives = 1) out;

layout(location = 0) perprimitiveEXT out mesh_output
{
	vec4 color;
} o_mesh_output[];

const vec4[3] c_positions = {
	vec4(-1.0,  1.0, 0.0, 1.0),
	vec4( 1.0,  1.0, 0.0, 1.0),
	vec4( 0.0, -1.0, 0.0, 1.0)
};

const vec4[3] c_colors = {
	vec4(0.0, 1.0, 0.0, 1.0),
	vec4(0.0, 0.0, 1.0, 1.0),
	vec4(1.0, 0.0, 0.0, 1.0)
};

void main()
{
    // Define one triangle.
	SetMeshOutputsEXT(3, 1);

    // Set position of each vertice.
	gl_MeshVerticesEXT[0].gl_Position = c_positions[0];
	gl_MeshVerticesEXT[1].gl_Position = c_positions[1];
	gl_MeshVerticesEXT[2].gl_Position = c_positions[2];

	const uint invocation_index = gl_LocalInvocationIndex;

    // Set associated to the primitive.
	o_mesh_output[invocation_index].color = c_colors[0];

    // Finally set triangle primitive indices.
	gl_PrimitiveTriangleIndicesEXT[invocation_index] =  uvec3(0, 1, 2);
}
