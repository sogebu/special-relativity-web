#version 300 es

in vec3 vert_position;
in vec4 vert_color;

out vec4 color;

uniform mat4 model;
uniform mat4 lorentz;
uniform mat4 view_perspective;

void main() {
    color = vert_color;
    vec4 model_vertex = model * vec4(vert_position, 1.0);
    model_vertex.w = -length(model_vertex.xyz);
    vec4 world_vertex = lorentz * model_vertex;
    gl_Position = view_perspective * vec4(world_vertex.xyz, 1.0);
}
