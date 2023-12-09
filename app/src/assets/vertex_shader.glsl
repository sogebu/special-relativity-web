#version 300 es

in vec3 vert_local_position;
in vec3 vert_world_position;
in vec3 vert_scale;
in vec4 vert_color;

out vec4 color;

uniform mat4 model;
uniform mat4 lorentz;
uniform mat4 view_perspective;

void main() {
    color = vert_color;

    vec3 v = vert_local_position * vert_scale + vert_world_position;
    vec4 model_vertex = model * vec4(v, 1.0);
    model_vertex.w = -length(model_vertex.xyz);
    vec4 world_vertex = lorentz * model_vertex;
    gl_Position = view_perspective * vec4(world_vertex.xyz, 1.0);
}