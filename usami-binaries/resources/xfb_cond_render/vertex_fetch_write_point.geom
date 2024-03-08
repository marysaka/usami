#version 450
layout (points) in;
layout(points, max_vertices = 1) out;
layout(location = 0, stream = 0, xfb_offset = 0, xfb_stride = 4, xfb_buffer = 0) out float output1;
layout(location = 1, stream = 1, xfb_offset = 0, xfb_stride = 4, xfb_buffer = 1) out float output2;
layout(location = 2, stream = 2, xfb_offset = 0, xfb_stride = 4, xfb_buffer = 2) out float output3;
layout(location = 3, stream = 3, xfb_offset = 0, xfb_stride = 4, xfb_buffer = 3) out float output4;
layout(push_constant) uniform PushConst {
    int stream;
} pushConst;
void main() {
    if (pushConst.stream == 0) {
        output1 = 1.0;
        EmitStreamVertex(0);
        EndStreamPrimitive(0);
    }
    if (pushConst.stream == 1) {
        output2 = 2.0;
        EmitStreamVertex(1);
        EndStreamPrimitive(1);
    }
    if (pushConst.stream == 2) {
        output3 = 3.0;
        EmitStreamVertex(2);
        EndStreamPrimitive(2);
    }
    if (pushConst.stream == 3) {
        output4 = 4.0;
        EmitStreamVertex(3);
        EndStreamPrimitive(3);
    }
gl_PointSize = 1.0f;
}