#version 330 core

in vec3 vertexColor;
in vec2 TexCoord;

out vec3 FragColor;
uniform sampler2D ourTexture;

void main() {
    FragColor = texture(ourTexture, TexCoord);
}