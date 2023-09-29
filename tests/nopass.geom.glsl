#version 450

layout(triangles) in;
layout(triangle_strip) out;
layout(max_vertices=3) out;

layout (location = 0) in Inputs {
    vec2 texcoord;
    vec4 baseColor;
} v_in[];

layout (location = 0) out Outputs {
    vec2 texcoord;
    vec4 baseColor;
};

void main()
{
    int layer = 2;
    for (int i = 0; i < 3; i++) {
        gl_Position = gl_in[i].gl_Position;
        texcoord = v_in[i].texcoord;
        baseColor = v_in[i].baseColor;
        gl_Layer = layer;
        EmitVertex();
    }
}