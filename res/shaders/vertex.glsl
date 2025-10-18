#version 460

layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec4 v_col;

out vec4 col;

void main() {
    gl_Position = vec4(v_pos.xyz, 1.0);
    col = v_col;
}
