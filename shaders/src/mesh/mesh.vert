#version 460
#extension GL_ARB_separate_shader_objects : enable

uniform float elapsed;
uniform mat4 perspectiveMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;

in vec3 inVertexPos;
in vec3 inNormal;
in vec2 inUV;
in vec4 inTangent;

out vec3 inoutNormal;
out vec4 inoutTangent;
out vec3 inoutWorldPos;
out vec2 inoutUV;

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
