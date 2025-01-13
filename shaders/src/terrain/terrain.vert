#version 460
#extension GL_ARB_separate_shader_objects : enable

uniform mat4 perspectiveMatrix;
uniform mat4 viewMatrix;
uniform mat4 partMatrix[320];

in vec3 inVertexPos;
in vec3 inNormal;
in vec4 inColorRoughness;
in uint inPartIndex;

out vec3 norm;
out vec3 worldPos;
out vec3 color;
out float roughness;

void main() {
  vec4 worldPosTmp4 = partMatrix[inPartIndex] * vec4(inVertexPos, 1.0);
  worldPos = worldPosTmp4.xyz;
  norm = normalize((partMatrix[inPartIndex] * vec4(inNormal, 0.0)).xyz);
  color = inColorRoughness.rgb;
  roughness = inColorRoughness.a;

  gl_Position = vec4(perspectiveMatrix * viewMatrix * worldPosTmp4);
}
