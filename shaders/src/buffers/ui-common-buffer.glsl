layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING) uniform commonDataBuffer {
    vec4 fontAtlasSmallData[255];
    vec4 fontAtlasMediumData[255];
    vec4 fontAtlasLargeData[255];
} commonData;
layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING_ATLAS_SMALL) uniform sampler2D atlasSmallTexture;
layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING_ATLAS_MEDIUM) uniform sampler2D atlasMediumTexture;
layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING_ATLAS_LARGE) uniform sampler2D atlasLargeTexture;
