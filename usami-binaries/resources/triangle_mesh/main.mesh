#version 450
#extension GL_EXT_mesh_shader : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(triangles, max_vertices = 3, max_primitives = 1) out;

layout(location = 0) out mesh_output
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

    // Set associated to each vertice.
	o_mesh_output[0].color = c_colors[0];
	o_mesh_output[1].color = c_colors[1];
	o_mesh_output[2].color = c_colors[2];

    // Finally set triangle primitive indices.
	gl_PrimitiveTriangleIndicesEXT[gl_LocalInvocationIndex] =  uvec3(0, 1, 2);
}
