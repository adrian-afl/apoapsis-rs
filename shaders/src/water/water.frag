#version 460
#extension GL_ARB_separate_shader_objects : enable

in vec3 norm;
in vec3 worldPos;

uniform vec3 waterColor;
uniform float wavesHeight;

layout (location = 0) out vec4 outColorRGBroughnessA;
layout (location = 1) out vec4 outNormalRGBdistanceA;
layout (location = 2) out vec4 outEmissionRGBmetalnessA;

#include "include/write-log-depth.glsl"

void main() {
  outColorRGBroughnessA = vec4(waterColor, 0.2);
  outNormalRGBdistanceA = vec4(normalize(norm), length(worldPos));
  outEmissionRGBmetalnessA = vec4(0.0);

  writeLogDepth(length(worldPos));
}
