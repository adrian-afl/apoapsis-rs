
#pragma once
uniform vec3 frustumTopLeft;
uniform vec3 frustumBottomLeft;
uniform vec3 frustumTopRight;
uniform vec3 frustumBottomRight;

vec3 reconstructCameraRay(vec2 uv, float dist){
    vec3 frustumConeBottomLeftToBottomRight = frustumBottomRight - frustumBottomLeft;
    vec3 frustumConeBottomLeftToTopLeft = frustumTopLeft - frustumBottomLeft;

    // uv.y = 1.0 - uv.y;
    vec3 dir = normalize(
        frustumBottomLeft
        + frustumConeBottomLeftToBottomRight * uv.x
        + frustumConeBottomLeftToTopLeft * uv.y
    );
    return dir * dist;
}