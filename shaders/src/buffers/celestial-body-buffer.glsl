layout(set = CELESTIAL_BUFFER_SET, binding = CELESTIAL_BUFFER_BINDING) uniform celestial {
    mat4 rotationMatrix;
    vec4 bodyCenter_zero;
    vec4 cloudsColor_zero;

    float terrainRadius;
    float waterRadius;
    float atmosphereStart;
    float cloudsMinHeight;

    float cloudsMaxHeight;
    float rayleighHeight;
    float rayleighDensity;
    float mieHeight;

    vec4 mieColor_mieDensity;

    vec4 starDirection_zero;
    vec4 starRadiance_zero;
} celestialData;

mat3 rotationMatrix = mat3(celestialData.rotationMatrix);
vec3 bodyCenter = celestialData.bodyCenter_zero.xyz;
vec3 cloudsColor  = celestialData.cloudsColor_zero.xyz;

float terrainRadius = celestialData.terrainRadius;
float waterRadius = celestialData.waterRadius;
float atmosphereStart = celestialData.atmosphereStart;
float cloudsMinHeight = celestialData.cloudsMinHeight;
float cloudsMaxHeight = celestialData.cloudsMaxHeight;
float rayleighHeight = celestialData.rayleighHeight;
float rayleighDensity = celestialData.rayleighDensity;
float mieHeight = celestialData.mieHeight;

vec3 mieColor = celestialData.mieColor_mieDensity.rgb;
float mieDensity = celestialData.mieColor_mieDensity.a;

vec3 starDirection = celestialData.starDirection_zero.xyz;
vec3 starRadiance = celestialData.starRadiance_zero.xyz;