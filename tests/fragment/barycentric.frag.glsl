#version 450
#extension GL_EXT_fragment_shader_barycentric : enable

layout(location = 0) out vec4 uFragColor;

void main(void)
{
    uFragColor = vec4(gl_BaryCoordEXT, 1.0);
}

