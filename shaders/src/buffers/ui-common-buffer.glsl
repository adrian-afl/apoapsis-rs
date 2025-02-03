layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING) buffer commonDataBuffer {
    vec4 resolution_zero_zero;
    vec4 fontAtlasSmallData[255];
    vec4 fontAtlasMediumData[255];
    vec4 fontAtlasLargeData[255];
} commonData;
layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING_ATLAS_SMALL) 
    uniform sampler2D atlasSmallTexture;
layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING_ATLAS_MEDIUM) 
    uniform sampler2D atlasMediumTexture;
layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING_ATLAS_LARGE) 
    uniform sampler2D atlasLargeTexture;
vec2 resolution = commonData.resolution_zero_zero.xy;