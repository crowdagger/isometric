#version 140
out vec4 color;
in vec2 v_tex_coords;
in vec3 v_normal;

uniform sampler2D tex;
uniform vec3 v_light;
uniform vec3 light_color;
uniform vec3 dark_color;

void main() {
    float brightness = dot(normalize(v_normal), normalize(v_light));
    vec4 ratio = vec4(mix(dark_color, light_color, brightness), 1.0);
    color = ratio * texture(tex, v_tex_coords);
}