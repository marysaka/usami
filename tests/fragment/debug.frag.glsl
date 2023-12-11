#version 450
#extension GL_EXT_mesh_shader : enable
#extension GL_EXT_shader_8bit_storage: require

layout (location=0) in vec4 customAttribute1;
layout (location=1) in flat float customAttribute2;
layout (location=2) in flat int customAttribute3;

layout (location=3) in perprimitiveEXT flat uvec4 customAttribute4;
layout (location=4) in perprimitiveEXT float customAttribute5;

layout (location=0) out vec4 outColor;

layout(binding = 0) buffer UniformBufferObject {
    uint data[0x1000];
} ubo;

void main ()
{
    bool goodPrimitiveID = (gl_PrimitiveID == 1000 || gl_PrimitiveID == 1001);
    bool goodViewportIndex = (gl_ViewportIndex == 1);
    bool goodCustom1 = (customAttribute1.x >= 0.25 && customAttribute1.x <= 0.5 &&
                        customAttribute1.y >= 0.5  && customAttribute1.y <= 1.0 &&
                        customAttribute1.z >= 10.0 && customAttribute1.z <= 20.0 &&
                        customAttribute1.w == 3.0);
    bool goodCustom2 = (customAttribute2 == 1.0 || customAttribute2 == 2.0);
    bool goodCustom3 = (customAttribute3 == 3 || customAttribute3 == 4);
    bool goodCustom4 = ((gl_PrimitiveID == 1000 && customAttribute4 == uvec4(100, 101, 102, 103)) ||
                        (gl_PrimitiveID == 1001 && customAttribute4 == uvec4(200, 201, 202, 203)));
    bool goodCustom5 = ((gl_PrimitiveID == 1000 && customAttribute5 == 6.0) ||
                        (gl_PrimitiveID == 1001 && customAttribute5 == 7.0));
    
    if (goodPrimitiveID && goodViewportIndex && goodCustom1 && goodCustom2 && goodCustom3 && goodCustom4 && goodCustom5) {
        outColor = vec4(0.0, 0.0, 1.0, 1.0);
    } else {
        outColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}