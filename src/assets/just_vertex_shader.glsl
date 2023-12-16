#version 300 es

in vec3 vert_local_position;

out vec4 color;

uniform vec4 uniform_color;
uniform mat4 model_view_perspective;

void main() {
    color = uniform_color;
    gl_Position = model_view_perspective * vec4(vert_local_position, 1.0);
}
