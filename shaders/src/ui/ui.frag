#version 460
#extension GL_ARB_separate_shader_objects : enable

#define ITEM_BUFFER_SET 0
#define ITEM_BUFFER_BINDING 0
#include "buffers/ui-item-buffer.glsl"

layout(set = 0, binding = 1) uniform sampler2D colorTexture;

#define COMMON_BUFFER_SET 1
#define COMMON_BUFFER_BINDING 0
#define COMMON_BUFFER_BINDING_ATLAS_SMALL 1
#define COMMON_BUFFER_BINDING_ATLAS_MEDIUM 2
#define COMMON_BUFFER_BINDING_ATLAS_LARGE 3
#include "buffers/ui-common-buffer.glsl"

layout (location = 0) in vec2 inoutUV;

layout (location = 0) out vec4 outColor;

float linearstep(float edge0, float edge1, float x) {
    return clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
}

void main() {
    vec4 c = color;
    if(useTexture) {
        c = texture(colorTexture, inoutUV); // TODO influence
    }
    // outColor = color;

    if(textLength > 0) {
        int letter_in_bounds = 0;
        float x_offset = 0;
        vec4 indices = commonData.fontAtlasSmallData[0];

        vec2 texSize = vec2(0.0);
        if(textFontSize == 1) 
            texSize = textureSize(atlasSmallTexture, 0);
        else if(textFontSize == 2) 
            texSize =  textureSize(atlasMediumTexture, 0);
        else if(textFontSize == 3) 
            texSize = textureSize(atlasLargeTexture, 0);
            
        vec2 size_pixels = size * resolution;
        int current_offset = int(inoutUV.x * size_pixels.x);
        for(; letter_in_bounds < textLength; letter_in_bounds++){
            uint letter = itemData.text[letter_in_bounds];

            if(textFontSize == 1) indices = commonData.fontAtlasSmallData[letter];
            else if(textFontSize == 2) indices = commonData.fontAtlasMediumData[letter];
            else if(textFontSize == 3) indices = commonData.fontAtlasLargeData[letter];
            if(x_offset + indices.z > current_offset) {
                float a = x_offset;
                float b = x_offset + indices.z;
                float m = linearstep(a, b, current_offset);
                
                int current_offset_y = int(inoutUV.y * size_pixels.y);
                ivec2 lookup = ivec2(
                    int(mix(indices.x, indices.x + indices.z, m)),
                    inoutUV.y * size_pixels.y
                );

                lookup.x -= 1; // VERY spooky
                lookup = clamp(lookup, ivec2(0.0, 0.0), ivec2(texSize));

                float textResult = 0.0;
                if(textFontSize == 1) 
                    textResult = texelFetch(atlasSmallTexture, lookup, 0).r;
                else if(textFontSize == 2) 
                    textResult = texelFetch(atlasMediumTexture, lookup, 0).r;
                else if(textFontSize == 3) 
                    textResult = texelFetch(atlasLargeTexture, lookup, 0).r;

                //textResult = smoothstep(0.4, 1.0, textResult);

                c = vec4(text_color.rgb, text_color.a * textResult);

                break;
            }

            x_offset += letter == 0 ? 10 : indices.z;
        }


        // float mult = inoutUV.x * float(textLength);
        // float letter_x = fract(mult);
        // // letter_index is from 0 to text length
        // uint letter_index = uint(min(1023.0, floor(mult)));
        
        // // letter is text letter index at position of letter_index
        // uint letter = itemData.text[letter_index];

        // vec4 indices = commonData.fontAtlasSmallData[letter];
        // if(textFontSize == 2) indices = commonData.fontAtlasMediumData[letter];
        // else if(textFontSize == 3) indices = commonData.fontAtlasLargeData[letter];

        // vec2 lookup = vec2(
        //     mix(indices.x, indices.x + indices.z, letter_x),
        //     mix(indices.y, indices.y + indices.w, inoutUV.y)
        // );

        // float textResult = 0.0;
        // if(textFontSize == 1) 
        //     textResult = texture(atlasSmallTexture, lookup / textureSize(atlasSmallTexture, 0), 0).r;
        // else if(textFontSize == 2) 
        //     textResult = texture(atlasMediumTexture, lookup / textureSize(atlasMediumTexture, 0), 0).r;
        // else if(textFontSize == 3) 
        //     textResult = texture(atlasLargeTexture, lookup / textureSize(atlasLargeTexture, 0), 0).r;

        // c = vec4(text_color.rgb, text_color.a * textResult) + vec4(0.5, 0.0, 0.0, 0.5);
        //c =  vec4(float(itemData.text[letter_index]) / 100.0, 0.0, 0.0, 1.0);
    }

    outColor = c;
}
