#version 300 es

in vec3 vert_local_position;
in vec3 vert_normal;

out vec4 color;
out vec3 normal;
out vec3 light_ray;

uniform vec4 uniform_color;
uniform mat4 model_view_projection_matrix;
uniform mat3 normal_matrix;

void main() {
    color = uniform_color;
    normal = normalize(normal_matrix * vert_normal);
    light_ray = normalize(normal_matrix * vec3(0.0, 1.0, 0.0));
    gl_Position = model_view_projection_matrix * vec4(vert_local_position, 1.0);
}
