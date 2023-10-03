#version 450

layout(points, invocations = 6) in;
layout(triangle_strip, max_vertices = 4) out;

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
    const int colorNdx = gl_InvocationID % 6;

    gl_Position = vec4(-1.0, -1.0, 0.0, 1.0);
    gl_Layer    = gl_InvocationID;
    gl_PointSize = 1.0;
    vert_color  = colors[colorNdx];
    EmitVertex();

    gl_Position = vec4(-1.0,  1.0, 0.0, 1.0);
    gl_Layer    = gl_InvocationID;
    gl_PointSize = 1.0;
    vert_color  = colors[colorNdx];
    EmitVertex();

    gl_Position = vec4( 0.0, -1.0, 0.0, 1.0);
    gl_Layer    = gl_InvocationID;
    gl_PointSize = 1.0;
    vert_color  = colors[colorNdx];
    EmitVertex();

    gl_Position = vec4( 0.0,  1.0, 0.0, 1.0);
    gl_Layer    = gl_InvocationID;
    gl_PointSize = 1.0;
    vert_color  = colors[colorNdx];
    EmitVertex();
    EndPrimitive();
}