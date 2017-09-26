#version 140
in vec3 position;
in float lighted;
in vec2 tex_coords;
in vec3 normal;
in float final_z;

uniform mat4 perspective;
uniform mat4 view;

out vec3 v_normal;
out float v_lighted;
out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    v_lighted = lighted;
    v_normal = normal;
    gl_Position = perspective * view * vec4(position, 1.0);
    gl_Position[2] = final_z / 1000.0;
}
