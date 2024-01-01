#version 300 es
precision highp float;

in vec4 color;
in vec3 normal;
in vec3 light_ray;

out vec4 frag_color;

void main() {
    float diffuse = max(dot(normal, light_ray), 0.0);
    frag_color = color * (0.5 + diffuse * 0.5);
}
