#pragma once

vec4 textureLinear(sampler2D tex, vec2 coord){
   vec2 texSize = vec2(textureSize(tex, 0));
   vec2 invTexSize = 1.0 / texSize;

    vec2 floorCoord = floor(coord * texSize) * invTexSize;
    vec2 fractCoord = fract(coord * texSize);

    vec3 offset = vec3(invTexSize, 0.0);

    vec4 sample0 = texture(tex, floorCoord);
    vec4 sample1 = texture(tex, floorCoord + offset.xz);
    vec4 sample2 = texture(tex, floorCoord + offset.zy);
    vec4 sample3 = texture(tex, floorCoord + offset.xy);

    return mix(
       mix(sample0, sample1, fractCoord.x), 
       mix(sample2, sample3, fractCoord.x)
    , fractCoord.y);
}
