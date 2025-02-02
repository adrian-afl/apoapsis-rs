#version 460
#extension GL_ARB_separate_shader_objects : enable

#define ITEM_BUFFER_SET 0
#define ITEM_BUFFER_BINDING 0
#include "buffers/ui-item-buffer.glsl"

#define COMMON_BUFFER_SET 1
#define COMMON_BUFFER_BINDING 0
#define COMMON_BUFFER_BINDING_ATLAS_SMALL 1
#define COMMON_BUFFER_BINDING_ATLAS_MEDIUM 2
#define COMMON_BUFFER_BINDING_ATLAS_LARGE 3
#include "buffers/ui-common-buffer.glsl"

layout (location = 0) in vec2 inVertexPos;
layout (location = 1) in vec2 inUV;

layout (location = 0) out vec2 inoutUV;

void main() {
  inoutUV = inUV;

  vec2 unorm = inVertexPos.xy * 0.5 + 0.5;
  vec2 sized = unorm * size;
  vec2 moved = sized + position;

  moved.y = 1.0 - moved.y;

//   gl_Position = vec4(sized, 0.0, 0.0);
  gl_Position = vec4(moved * 2.0 - 1.0, 0.0, 1.0);
}
