#version 450

layout(points) in;
layout(triangle_strip, max_vertices = 24) out;

layout(location = 0) out vec4 vert_color;

out gl_PerVertex {
    vec4 gl_Position;
    float gl_PointSize;
};

void main(void)
{
    const vec4 colors[6] = vec4[6](vec4(1.0, 1.0, 1.0, 1.0),
                                   vec4(1.0, 0.0, 0.0, 1.0),
                                   vec4(0.0, 1.0, 0.0, 1.0),
                                   vec4(0.0, 0.0, 1.0, 1.0),
                                   vec4(1.0, 1.0, 0.0, 1.0),
                                   vec4(1.0, 0.0, 1.0, 1.0));

    for (int layerNdx = 0; layerNdx < 6; ++layerNdx) {
        const int colorNdx = layerNdx % 6;

        gl_Position = vec4(-1.0, -1.0, 0.0, 1.0);
        gl_Layer    = layerNdx;
        vert_color  = colors[colorNdx];
        gl_PointSize = 1.0;
        EmitVertex();

        gl_Position = vec4(-1.0,  1.0, 0.0, 1.0);
        gl_Layer    = layerNdx;
        vert_color  = colors[colorNdx];
        gl_PointSize = 1.0;
        EmitVertex();

        gl_Position = vec4( 0.0, -1.0, 0.0, 1.0);
        gl_Layer    = layerNdx;
        vert_color  = colors[colorNdx];
        gl_PointSize = 1.0;
        EmitVertex();

        gl_Position = vec4( 0.0,  1.0, 0.0, 1.0);
        gl_Layer    = layerNdx;
        vert_color  = colors[colorNdx];
        gl_PointSize = 1.0;
        EmitVertex();
        EndPrimitive();
    };
}