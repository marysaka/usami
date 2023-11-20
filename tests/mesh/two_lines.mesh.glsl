#version 450
#extension GL_EXT_mesh_shader : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(lines, max_vertices = 256, max_primitives = 256) out;

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
    // Define two lines.
	SetMeshOutputsEXT(2, 2);

    // Set position of each vertice.
	gl_MeshVerticesEXT[0].gl_Position = c_positions[0];
	gl_MeshVerticesEXT[1].gl_Position = c_positions[1];

	gl_MeshVerticesEXT[2].gl_Position = c_positions[1];
	gl_MeshVerticesEXT[3].gl_Position = c_positions[2];


    // Set associated to each vertice.
	o_mesh_output[0].color = c_colors[0];
	o_mesh_output[1].color = c_colors[1];

	o_mesh_output[2].color = c_colors[1];
	o_mesh_output[3].color = c_colors[2];


    // Finally set triangle primitive indices.
	gl_PrimitiveLineIndicesEXT[0] =  uvec2(0, 1);
	gl_PrimitiveLineIndicesEXT[1] =  uvec2(2, 3);
}
