#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 col;

out vec3 color;

void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    color = col;
}