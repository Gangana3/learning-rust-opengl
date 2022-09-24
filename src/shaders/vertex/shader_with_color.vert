#version 330 core

//Input is received from the vertex buffer object
layout (location=0) in vec3 aPosition;
layout (location=1) in vec3 customColor;

out vec3 vertexColor;

void main() {
    gl_Position = vec4(aPosition.x, aPosition.y, aPosition.z, 1.0f);
    vertexColor = customColor;
}