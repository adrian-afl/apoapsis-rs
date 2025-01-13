#version 460
#extension GL_ARB_separate_shader_objects : enable

uniform mat4 modelMatrix;

uniform vec3 color;
uniform uint useColorTexture;
uniform float colorTextureScale;

uniform float roughness;
uniform uint useRoughnessTexture;
uniform float roughnessTextureScale;

uniform float metalness;
uniform uint useMetalnessTexture;
uniform float metalnessTextureScale;

uniform vec3 emission;
uniform uint useEmissionTexture;
uniform float emissionTextureScale;

uniform uint useNormalTexture;
uniform float normalTextureScale;

uniform uint useBumpTexture;
uniform float bumpTextureScale;

uniform sampler2D colorTexture;
uniform sampler2D roughnessTexture;
uniform sampler2D metalnessTexture;
uniform sampler2D emissionTexture;
uniform sampler2D normalTexture;
uniform sampler2D bumpTexture;

in vec3 inoutNormal;
in vec4 inoutTangent;
in vec3 inoutWorldPos;
in vec2 inoutUV;

layout (location = 0) out vec4 outColorRGBroughnessA;
layout (location = 1) out vec4 outNormalRGBdistanceA;
layout (location = 2) out vec4 outEmissionRGBmetalnessA;

#include "include/write-log-depth.glsl"

void main() {
  vec3 resultColor = vec3(0.0);
  vec3 resultNormal = vec3(0.0);
  vec3 resultEmission = vec3(0.0);
  float resultRoughness = 0.0;
  float resultMetalness = 0.0;

  if(useColorTexture == 0u){
    resultColor = color;
  } else {
    resultColor = texture(colorTexture, inoutUV * colorTextureScale).rgb;
  }

  if(useEmissionTexture == 0u){
    resultEmission = emission;
  } else {
    resultEmission = texture(emissionTexture, inoutUV * emissionTextureScale).rgb;
  }

  if(useRoughnessTexture == 0u){
    resultRoughness = roughness;
  } else {
    resultRoughness = texture(roughnessTexture, inoutUV * roughnessTextureScale).r;
  }

  if(useMetalnessTexture == 0u){
    resultMetalness = metalness;
  } else {
    resultMetalness = texture(metalnessTexture, inoutUV * metalnessTextureScale).r;
  }

  vec3 normalMap = vec3(1.0);
  if(useNormalTexture == 1u){
    vec3 map = normalize(texture(normalTexture, inoutUV * normalTextureScale).rgb * 2.0 - 1.0);
    map.r *= -1.0;
    map.g *= -1.0;
    normalMap *= map;
  }

  if(useBumpTexture == 1u){
    vec2 scaledUV = inoutUV * bumpTextureScale;
    vec2 dsp = 1.0 / vec2(textureSize(bumpTexture, 0));
    float bc = texture(bumpTexture, scaledUV).r;
    float bdx = bc - texture(bumpTexture, scaledUV+vec2(dsp.x, 0)).r;
    float bdy = bc - texture(bumpTexture, scaledUV+vec2(0, dsp.y)).r;

    normalMap *= normalize(vec3(bdx * 3.1415, bdy * 3.1415, max(0.0, 1.0 - bdx - bdy)));
  }

  if(useNormalTexture == 1u || useBumpTexture == 1u){
    mat3 TBN = mat3(
        normalize(inoutTangent.xyz),
        normalize(cross(inoutNormal.xyz, inoutTangent.xyz)) * inoutTangent.w,
        normalize(inoutNormal.xyz)
    );
    resultNormal = TBN * normalMap;
  } else {
    resultNormal = normalize(inoutNormal.xyz);
  }
  
  outColorRGBroughnessA = vec4(resultColor, resultRoughness);
  outNormalRGBdistanceA = vec4(resultNormal, length(inoutWorldPos));
  outEmissionRGBmetalnessA = vec4(resultEmission, resultMetalness);

  writeLogDepth(length(inoutWorldPos));
}
