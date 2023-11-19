#version 450
#extension GL_EXT_mesh_shader : require

struct task_payload
{
  uint value;
  double big_value;
  bool tiny_value;
};

taskPayloadSharedEXT task_payload task_data;

void main()
{
  task_data.value = 0x42;
  task_data.big_value = 66.0;
  task_data.tiny_value = true;

	EmitMeshTasksEXT(2, 3, 4);
}

