#version 460
#extension GL_ARB_separate_shader_objects : enable

#include "uniforms/celestial-body.glsl"

uniform mat4 perspectiveMatrix;
uniform mat4 viewMatrix;
uniform mat4 partMatrix[320];

in vec3 inVertexPos;
in uint inPartIndex;

out vec3 norm;
out vec3 worldPos;

void main() {
  vec4 worldPosTmp4 = partMatrix[inPartIndex] * vec4(inVertexPos, 1.0);
  worldPos = worldPosTmp4.xyz;
  norm = normalize(worldPos - bodyCenter);

  gl_Position = vec4(perspectiveMatrix * viewMatrix * worldPosTmp4);
}
