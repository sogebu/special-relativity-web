#version 300 es

in vec3 vert_local_position;
in vec3 vert_normal;

out vec4 color;

uniform vec4 uniform_color;
uniform mat4 model_view_projection_matrix;
uniform mat3 normal_matrix;

void main() {
    vec3 light_dir = normalize(vec3(1.0, 1.0, -1.0)) * 1.5;
    vec3  inv_light = normal_matrix * light_dir;
    float diffuse  = clamp(dot(vert_normal, inv_light), 0.6, 2.0);
    color = vec4(uniform_color.rgb * diffuse, uniform_color.a);

    gl_Position = model_view_projection_matrix * vec4(vert_local_position, 1.0);
}
