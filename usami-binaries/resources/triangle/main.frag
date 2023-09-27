#version 450

layout(location = 0) in vec4 o_color;
layout(location = 1) in vec3 frag_pos;
layout(location = 0) out vec4 uFragColor;

const vec3 lightDir = vec3(0.424, 0.566, 0.707);

void main(void)
{
    vec3 dX = dFdx(frag_pos);
    vec3 dY = dFdy(frag_pos);
    vec3 normal = normalize(cross(dX,dY));

    float light = max(0.0, dot(lightDir, normal));

    vec4 result = vec4(0.0);

    for (int i = 0; i < int(dX.x); i++) {
        result += light * o_color * i;
    }

    uFragColor = result;
}