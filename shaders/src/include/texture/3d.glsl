#include "include/texture/linear.glsl"

vec4 texture2Das3D(sampler2D tex, vec3 texCoord, float size) {
    vec3 repeated = mod(texCoord, vec3(1.0)); // repeat 
    float x = repeated.x / size + floor(repeated.z * size) / size;
    float zfract = fract(repeated.z * size);
    vec2 coorda = vec2(x, repeated.y);
    vec2 coordb = vec2(x + 1.0/size, repeated.y);
    vec4 a = textureLinear(tex, coorda);
    vec4 b = textureLinear(tex, coordb);
    return mix(a, b, zfract);
}

vec3 uvTo3D(vec2 uv2d, float size) {
    float y = uv2d.y;

    float sliceSize = 1.0 / size;
    float x = mod(uv2d.x, sliceSize);
    float z = (uv2d.x - x);

    return vec3(x * size, y, z);
}
