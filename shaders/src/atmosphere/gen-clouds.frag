#version 460
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform ubo {
    vec4 seed;
    vec4 elapsed_frequency_zero_zero;
} uniforms;

vec3 seed = uniforms.seed.xyz;
float elapsed = uniforms.elapsed_frequency_zero_zero.x;
float frequency = uniforms.elapsed_frequency_zero_zero.y;

layout(location = 0) in vec2 UV;

layout (location = 0) out float result;

#include "include/polar.glsl"
#include "include/noise/super-3d.glsl"

float cloudsFBM(vec3 p, int iterations){
    float a = 0.0;
    float w = 0.5;
    for(int i=0;i<iterations;i++){
        float x = abs(0.5 - supernoise3D(p))*2.0;
        a += x * w;
        p = p * 2.9 + p * a * 0.001;
        w *= 0.60;
    }
    return a;
}

void main() {
    vec3 normal = normalize(polarToXyz(UV));
    result = cloudsFBM(normal * frequency + seed.xyz + elapsed * 0.00001, 8);
}
