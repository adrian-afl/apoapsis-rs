
float getAltitude(vec3 point){
    float dist = distance(point, bodyCenter);
    return dist - atmosphereStart;
}

float rand2d(vec2 co){
    return fract(sin(dot(co.xy,vec2(12.9898,78.233))) * 43758.5453);
}

struct RayleighMieResult {
    vec3 rayleigh;
    vec4 mie;
};

struct CloudsResult {
    float coverage;
    float distance;
    float relativeHeight;
    vec3 normal;
};