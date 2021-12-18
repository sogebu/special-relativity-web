#version 300 es

in vec3 vert_position;
in vec4 vert_color;

out vec4 color;

void main() {
    color = vert_color;
    gl_Position = vec4(vert_position, 1.0);
}
