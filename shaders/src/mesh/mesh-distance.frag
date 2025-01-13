#version 460
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) out float outDistance;

void main() {
  outDistance = gl_FragCoord.z;
}
