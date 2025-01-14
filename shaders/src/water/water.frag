#version 460
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec3 norm;
layout (location = 1) in vec3 worldPos;

layout(set = 0, binding = 0) uniform ubo {
    mat4 perspectiveMatrix;
    mat4 viewMatrix;
    vec4 waterColor_zero;
    vec4 bodyCenter_zero;
    mat4 partMatrix[320];
} uniforms;

//uniform float wavesHeight;

layout (location = 0) out vec4 outColorRGBroughnessA;
layout (location = 1) out vec4 outNormalRGBdistanceA;
layout (location = 2) out vec4 outEmissionRGBmetalnessA;

#include "include/write-log-depth.glsl"

void main() {
  outColorRGBroughnessA = vec4(uniforms.waterColor_zero.rgb, 0.2);
  outNormalRGBdistanceA = vec4(normalize(norm), length(worldPos));
  outEmissionRGBmetalnessA = vec4(0.0);

  writeLogDepth(length(worldPos));
}
