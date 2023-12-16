#version 300 es

in vec3 vert_local_position;

out vec4 color;

uniform vec4 uniform_color;
uniform mat4 model;
uniform mat4 lorentz;
uniform mat4 view_perspective;

void main() {
    color = uniform_color;

    vec4 model_vertex = model * vec4(vert_local_position, 1.0);
    model_vertex.w = -length(model_vertex.xyz);
    vec4 world_vertex = lorentz * model_vertex;
    gl_Position = view_perspective * vec4(world_vertex.xyz, 1.0);
}
