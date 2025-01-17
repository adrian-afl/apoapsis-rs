#version 460
#extension GL_ARB_separate_shader_objects : enable

#define MESH_BUFFER_SET 0
#define MESH_BUFFER_BINDING 0
#include "buffers/mesh-buffer.glsl"

#define COMMON_BUFFER_SET 1
#define COMMON_BUFFER_BINDING 0
#include "buffers/common-buffer.glsl"

layout (location = 0) in vec3 inVertexPos;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec2 inUV;
layout (location = 3) in vec4 inTangent;

layout (location = 0) out vec3 inoutNormal;
layout (location = 1) out vec4 inoutTangent;
layout (location = 2) out vec3 inoutWorldPos;
layout (location = 3) out vec2 inoutUV;

void main() {
  inoutUV = inUV;

  inoutNormal = normalize(vec4(modelMatrix * vec4(inNormal, 0.0)).xyz);
  inoutTangent = vec4(
    normalize(vec4(modelMatrix * vec4(inTangent.xyz, 0.0)).xyz), 
    inTangent.w
  );

  vec4 tmpWorldPosVec4 = modelMatrix * vec4(inVertexPos, 1.0);
  inoutWorldPos = tmpWorldPosVec4.xyz;

  gl_Position = vec4(perspectiveMatrix * viewMatrix * tmpWorldPosVec4);
}
