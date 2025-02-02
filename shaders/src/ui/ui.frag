#version 460
#extension GL_ARB_separate_shader_objects : enable

#define ITEM_BUFFER_SET 0
#define ITEM_BUFFER_BINDING 0
#include "buffers/ui-item-buffer.glsl"

layout(set = 0, binding = 1) uniform sampler2D colorTexture;

layout (location = 0) in vec2 inoutUV;

layout (location = 0) out vec4 outColor;

void main() {
    vec4 c = color;
    if(useTexture) {
        c = texture(colorTexture, inoutUV); // TODO influence
    }
    // outColor = color;
    outColor = color;
}
