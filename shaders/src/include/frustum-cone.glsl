#pragma once
vec3 reconstructCameraRay(vec2 uv, float dist){

    // uv.y = 1.0 - uv.y;
    //opengl
    // vec3 frustumConeBottomLeftToBottomRight = frustumBottomRight - frustumBottomLeft;
    // vec3 frustumConeBottomLeftToTopLeft = frustumTopLeft - frustumBottomLeft;
    // vec3 dir = normalize(
    //     frustumBottomLeft
    //     + frustumConeBottomLeftToBottomRight * uv.x
    //     + frustumConeBottomLeftToTopLeft * uv.y
    // );
    //vulkan compute where 0x0 is top left
    vec3 frustumConeTopLeftToTopRight = frustumTopRight - frustumTopLeft;
    vec3 frustumConeTopLeftToBottomLeft = frustumBottomLeft - frustumTopLeft;
    vec3 dir = normalize(
        frustumTopLeft
        + frustumConeTopLeftToTopRight * uv.x
        + frustumConeTopLeftToBottomLeft * uv.y
    );
    return dir * dist;
}