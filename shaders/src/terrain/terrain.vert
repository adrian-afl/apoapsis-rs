#version 460
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) buffer readonly ubo {
    dmat4 partMatrix[];
} data;

#define COMMON_BUFFER_SET 1
#define COMMON_BUFFER_BINDING 0
#include "buffers/common-buffer.glsl"

layout (location = 0) in dvec4 inVertexPos;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec4 inColorRoughness;
layout (location = 3) in uint inPartIndex;

layout (location = 0) out vec3 norm;
layout (location = 1) out vec3 worldPos;
layout (location = 2) out vec3 color;
layout (location = 3) out float roughness;

void main() {
  dvec4 worldPosTmp4 = data.partMatrix[inPartIndex] * dvec4(inVertexPos.xyz, 1.0);
  worldPos = vec3(worldPosTmp4.xyz);
  norm = vec3(normalize((data.partMatrix[inPartIndex] * vec4(inNormal, 0.0)).xyz));
  color = inColorRoughness.rgb;
  roughness = inColorRoughness.a;

  gl_Position = vec4(perspectiveMatrix * viewMatrix * vec4(worldPosTmp4));
}
