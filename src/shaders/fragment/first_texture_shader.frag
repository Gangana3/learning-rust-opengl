#version 330 core

in vec3 vertexColor;
in vec2 TexCoord;

out vec4 FragColor;

uniform sampler2D texture1;

void main() {
    // Mix the texture with the given color from the vertex shader
    FragColor = texture(texture1, TexCoord) * vec4(vertexColor, 1.0);
}