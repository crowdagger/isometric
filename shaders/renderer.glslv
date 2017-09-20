#version 140
in vec3 position;
out vec2 v_tex_coords;

uniform mat4 perspective;
uniform mat4 view;
in vec2 tex_coords;
in vec3 normal;
out vec3 v_normal;

void main() {
    v_tex_coords = tex_coords;
    v_normal = normal;
    gl_Position = perspective * view * vec4(position, 1.0);
}
