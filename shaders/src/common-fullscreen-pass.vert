#version 460
#extension GL_ARB_separate_shader_objects : enable

in vec2 inVertexPos;
in vec2 inUV;

out vec2 UV;

void main() {
  gl_Position = vec4(inVertexPos, 0.0, 1.0);
  UV = inUV;
}
