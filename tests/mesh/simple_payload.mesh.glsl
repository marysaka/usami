#version 450
#extension GL_EXT_mesh_shader : require

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(triangles, max_vertices = 3, max_primitives = 42) out;

struct task_payload
{
  uint value;
  double big_value;
  bool tiny_value;
};

taskPayloadSharedEXT task_payload task_data;

void main()
{
	SetMeshOutputsEXT(3, task_data.value);
}

