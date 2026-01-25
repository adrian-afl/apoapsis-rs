layout(set = COMMON_BUFFER_SET, binding = COMMON_BUFFER_BINDING) uniform commonDataBuffer {
    mat4 perspectiveMatrix;
    mat4 viewMatrix;

    vec4 frustumTopLeft_zero;
    vec4 frustumBottomLeft_zero;
    vec4 frustumTopRight_zero;
    vec4 frustumBottomRight_zero;

    vec4 elapsed_debugmode_zero_zero;
} commonData;

float elapsed = commonData.elapsed_debugmode_zero_zero.x;

mat4 perspectiveMatrix = commonData.perspectiveMatrix;
mat4 viewMatrix = commonData.viewMatrix;

vec3 frustumTopLeft = commonData.frustumTopLeft_zero.xyz;
vec3 frustumBottomLeft = commonData.frustumBottomLeft_zero.xyz;
vec3 frustumTopRight = commonData.frustumTopRight_zero.xyz;
vec3 frustumBottomRight = commonData.frustumBottomRight_zero.xyz;

#define IS_DEBUG_MODE_DISTANCE (commonData.elapsed_debugmode_zero_zero.y == 1.0)
// #define IS_DEBUG_MODE_DISTANCE true