#version 460

in vec4 col;
in vec3 normal;
in vec2 uv;

uniform sampler2D u_texture;

out vec4 frag_col;

void main() {
    frag_col = texture(u_texture, uv) * col;
}
