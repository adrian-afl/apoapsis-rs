#version 460
#extension GL_ARB_separate_shader_objects : enable

uniform mat4 perspectiveMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;

in vec3 inVertexPos;
in vec3 inNormal;
in vec2 inUV;

void main() {
  gl_Position = vec4(perspectiveMatrix * viewMatrix * modelMatrix * vec4(inVertexPos, 1.0));
}
