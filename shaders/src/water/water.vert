#version 460
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform ubo {
    vec4 waterColor_zero;
    vec4 bodyCenter_zero;
    mat4 partMatrix[320];
} data;

#define COMMON_BUFFER_SET 1
#define COMMON_BUFFER_BINDING 0
#include "buffers/common-buffer.glsl"

layout (location = 0)in vec3 inVertexPos;
layout (location = 1)in uint inPartIndex;

layout (location = 0) out vec3 norm;
layout (location = 1) out vec3 worldPos;
layout (location = 2) out vec3 waterColor;

void main() {
  vec4 worldPosTmp4 = data.partMatrix[inPartIndex] * vec4(inVertexPos, 1.0);
  worldPos = worldPosTmp4.xyz;
  norm = normalize(worldPos - data.bodyCenter_zero.rgb);
  waterColor = data.waterColor_zero.rgb;

  gl_Position = vec4(perspectiveMatrix * viewMatrix * worldPosTmp4);
}
