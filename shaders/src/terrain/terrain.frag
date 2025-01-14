#version 460
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec3 norm;
layout (location = 1) in vec3 worldPos;
layout (location = 2) in vec3 color;
layout (location = 3) in float roughness;

layout (location = 0) out vec4 outColorRGBroughnessA;
layout (location = 1) out vec4 outNormalRGBdistanceA;
layout (location = 2) out vec4 outEmissionRGBmetalnessA;

#include "include/write-log-depth.glsl"

void main() {  
  outColorRGBroughnessA = vec4(color, roughness);
  outNormalRGBdistanceA = vec4(normalize(norm), length(worldPos));
  outEmissionRGBmetalnessA = vec4(0.0);

  writeLogDepth(length(worldPos));
}
