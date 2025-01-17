#version 460
#extension GL_ARB_separate_shader_objects : enable

#define MESH_BUFFER_SET 0
#define MESH_BUFFER_BINDING 0
#include "buffers/mesh-buffer.glsl"

#define COMMON_BUFFER_SET 1
#define COMMON_BUFFER_BINDING 0
#include "buffers/common-buffer.glsl"

layout (location = 0) in vec3 inVertexPos;

void main() {
  gl_Position = vec4(perspectiveMatrix * viewMatrix * modelMatrix * vec4(inVertexPos, 1.0));
}
