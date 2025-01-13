#pragma once

vec4 textureBlur(sampler2D tex, vec2 coord, int range){
    vec4 accu = vec4(0.0);
    float w = 0.0;
    vec2 texSize = vec2(textureSize(tex, 0));
    vec2 invTexSize = 1.0 / texSize;
    for(int y=-range;y<=range;y++){
        for(int x=-range;x<=range;x++){
            vec2 offset = vec2(float(x), float(y)) * invTexSize;
            accu += texture(tex, coord + offset);
            w += 1.0;
        }   
    }
    return accu / w;
}