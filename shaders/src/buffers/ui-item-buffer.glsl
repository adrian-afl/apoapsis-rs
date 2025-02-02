layout(set = ITEM_BUFFER_SET, binding = ITEM_BUFFER_BINDING) buffer meshDataBuffer {
    vec4 size_position;
    vec4 orientation_zero_zero;
    vec4 color;
    vec4 text_color;
    uvec4 useTexture_textLength_textFontSize;
    uint text[1024];
} itemData;

vec2 size = itemData.size_position.xy;
vec2 position = itemData.size_position.zw;
vec4 color = itemData.color;
vec4 text_color = itemData.text_color;
bool useTexture = itemData.useTexture_textLength_textFontSize.x > 0;
uint textLength = itemData.useTexture_textLength_textFontSize.y;
uint textFontSize = itemData.useTexture_textLength_textFontSize.z;