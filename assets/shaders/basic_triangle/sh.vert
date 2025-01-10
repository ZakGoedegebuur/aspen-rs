#version 450

// in per vert
layout(location = 0) in vec2 pos;
layout(location = 1) in vec3 color;

// out per vert
layout(location = 0) out vec3 o_color;

void main() {
    gl_Position = pos;
    o_color = color;
}  