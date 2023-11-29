#version 450
#extension GL_EXT_mesh_shader : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(points, max_vertices = 256, max_primitives = 256) out;

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
	SetMeshOutputsEXT(1, 1);

	gl_MeshVerticesEXT[0].gl_PointSize = 42.0f;
	//gl_MeshVerticesEXT[0].gl_Position = c_positions[0];

    // Finally set triangle primitive indices.
	//gl_PrimitivePointIndicesEXT[0] = 0;
}
