#version 460
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform ubo {
    mat4 lightPerspectiveMatrix;
    mat4 lightViewMatrix;
    vec4 lightColor_zero;
    vec4 lightCameraRelativePosition_zero;
} uniforms;

layout(set = 0, binding = 1) uniform sampler2D colorTexture;
layout(set = 0, binding = 2) uniform sampler2D distanceTexture;
layout(set = 0, binding = 3) uniform sampler2D worldPosTexture;
layout(set = 0, binding = 4) uniform sampler2D normalTexture;

mat4 lightPerspectiveMatrix = uniforms.lightPerspectiveMatrix;
mat4 lightViewMatrix = uniforms.lightViewMatrix;

vec3 lightColor = uniforms.lightColor_zero.rgb;
vec3 lightCameraRelativePosition = uniforms.lightCameraRelativePosition_zero.xyz;

layout (location = 0) in vec2 UV;

layout (location = 0) out vec4 outColor;

float smoothShadow(vec3 ndc){
    float result = 0.0;
    float weight = 0.0;
    vec2 texSize = vec2(textureSize(distanceTexture, 0));
    for(int x = -2; x <= 2;x++){
        for(int y = -2; y <= 2;y++){
            vec2 offset = vec2(float(x), float(y)) / texSize.xy;
            float lightProjectionDepth = texture(distanceTexture, ndc.xy + offset).r;
            result += smoothstep(0.00, 0.0001, lightProjectionDepth - ndc.z);
            weight += 1.0;
        }
    }
    return result/weight;
}

void main() {
  vec3 color = texture(colorTexture, UV).rgb;
  vec3 mainCameraSpaceWorldPos = texture(worldPosTexture, UV).rgb;
  vec3 worldPos = mainCameraSpaceWorldPos - lightCameraRelativePosition;
  vec3 normal = texture(normalTexture, UV).rgb;
  if(length(normal) < 0.1){
      discard;
      return;
  }

  vec4 lightClipSpace = lightPerspectiveMatrix * lightViewMatrix * vec4(worldPos, 1.0);
  vec3 ndc = (lightClipSpace.xyz / lightClipSpace.w) * 0.5 + 0.5;

  float shadow = smoothShadow(ndc);

  vec3 lightDirection = normalize((lightPerspectiveMatrix * lightViewMatrix * vec4(0.0, 0.0, -1.0, 0.0)).xyz);

  float lambertian = max(0.0, dot(normal, lightDirection));

  outColor = vec4(color * shadow * lambertian, length(mainCameraSpaceWorldPos));
}
