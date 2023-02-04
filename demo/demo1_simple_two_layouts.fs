#version 330 core

out vec4 final_color;
in vec3 color;

void main() {
    final_color = vec4(color, 0);
}
