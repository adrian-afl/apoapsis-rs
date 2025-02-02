struct FontAtlasIndices {
    float x;
    float y;
    float w;
    float h;
};

layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING) uniform commonDataBuffer {
    FontAtlasIndices fontAtlasSmallData[255];
    FontAtlasIndices fontAtlasMediumData[255];
    FontAtlasIndices fontAtlasLargeData[255];
} commonData;

FontAtlasIndices[] fontAtlasSmallData = commonData.fontAtlasSmallData;
FontAtlasIndices[] fontAtlasMediumData = commonData.fontAtlasMediumData;
FontAtlasIndices[] fontAtlasLargeData = commonData.fontAtlasLargeData;