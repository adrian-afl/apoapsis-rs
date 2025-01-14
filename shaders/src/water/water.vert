#version 460
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform ubo {
    mat4 perspectiveMatrix;
    mat4 viewMatrix;
    vec4 waterColor;
    vec4 bodyCenter_zero;
    mat4 partMatrix[320];
} uniforms;

layout (location = 0)in vec3 inVertexPos;
layout (location = 1)in uint inPartIndex;

layout (location = 0) out vec3 norm;
layout (location = 1) out vec3 worldPos;

void main() {
  vec4 worldPosTmp4 = uniforms.partMatrix[inPartIndex] * vec4(inVertexPos, 1.0);
  worldPos = worldPosTmp4.xyz;
  norm = normalize(worldPos - uniforms.bodyCenter_zero.rgb);

  gl_Position = vec4(uniforms.perspectiveMatrix * uniforms.viewMatrix * worldPosTmp4);
}
