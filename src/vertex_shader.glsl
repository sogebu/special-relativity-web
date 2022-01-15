#version 300 es

in vec3 vert_position;
in vec4 vert_color;

out vec4 color;

uniform mat4 matrix;

void main() {
    color = vert_color;
    gl_Position = matrix * vec4(vert_position, 1.0);
}
