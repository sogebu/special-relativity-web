#version 300 es
precision highp float;

in vec4 color;
in vec3 normal;
in vec3 light_ray;

out vec4 frag_color;

void main() {
    float diffuse = max(dot(normal, light_ray), 0.0);

    vec3 rgb =  color.rgb * (diffuse * 0.5 + 0.5);
    frag_color = vec4(rgb, color.a);
}
