#version 460
#extension GL_ARB_separate_shader_objects : enable

in vec2 UV;

#include "include/frustum-cone.glsl"
#include "uniforms/celestial-body.glsl"
#include "uniforms/common.glsl"
#include "uniforms/star.glsl"
#include "include/sphere-raytracing.glsl"




uniform sampler2D gBufferColorRGBroughnessA;
uniform sampler2D gBufferNormalRGBdistanceA;
uniform sampler2D gBufferEmissionRGBmetalnessA;

uniform sampler2D cloudsLowFreqTextureDensityR;
uniform sampler2D cloudsHighFreqTextureDensityR;

layout (location = 0) out vec4 outAdditiveRGB;
layout (location = 1) out vec4 outAlphaRGBA;

#include "atmosphere/common.glsl"
#include "atmosphere/clouds.glsl"
#include "atmosphere/rayleigh-mie.glsl"
#include "atmosphere/sun-flare.glsl"

float quickSSAO(float uvradius) {
    float res = 0.0;
    float ws = 0.0;
    vec4 center = textureLod(gBufferNormalRGBdistanceA, UV, 0.0);
    float centerDist = center.a;
    vec3 centerDir = reconstructCameraRay(UV, 1.0);
    vec3 posCenter = centerDir * centerDist;
    vec3 centerNormal = center.rgb;
    //float kindOfFresnel = 1.0 - max(0.0, dot(centerDir, -centerNormal));
    for(int r = 0; r < 5; r++){
        float fr = float(r + 1) / 5.0;
        for(int i = 0; i < 5; i++){
            float fi = float(i);
            float x = sin(fi);
            float y = cos(fi);
            vec2 v = UV + uvradius * (vec2(x, y) * fr);
            vec4 data = textureLod(gBufferNormalRGBdistanceA, v, 6.0);
            float d = data.a;
            vec3 n = data.rgb;
            vec3 p = reconstructCameraRay(v, 1.0) * d;
            // float dt1 = max(0.0, dot(n, -centerNormal));
            float dt2 = max(0.0, dot(normalize(p - posCenter), centerNormal));
            float w = max(0.0, 1.0 - abs(d - centerDist)) / fr;
            float occlu = 1.0 - dt2;
            res += occlu * w;
            ws += w;
        }
    }
    float ao = ws == 0.0 ? 1.0 : res / ws;
    return max(0.0, pow(ao, 5.0));
    //return mix(1.0, ws == 0.0 ? 1.0 : res / ws, kindOfFresnel);
}

void main() {
    vec3 dir = reconstructCameraRay(UV, 1.0);
    float geometryHit = texture(gBufferNormalRGBdistanceA, UV).a;
    
    CloudsResult cloudsResult = clouds(dir, geometryHit);
    RayleighMieResult rayleighMieResult = rayleighMie(dir, geometryHit, cloudsResult);

    Sphere atmoSphere = Sphere(bodyCenter, atmosphereStart + max(rayleighHeight, mieHeight));
    vec2 atmosphereHitsFromCamera = rsi2X(vec3(0.0), dir, atmoSphere);
    vec2 atmosphereHitsFromClouds = rsi2X(cloudsResult.distance * dir, starDirection, atmoSphere);
    float atmosphereHitFromClouds = getForwardHitOrZero(atmosphereHitsFromClouds);

    vec3 sun = vec3(10.0) * flare(dir) * (1.0 - cloudsResult.coverage);
    if(hits(atmosphereHitsFromCamera.x) && hits(atmosphereHitsFromCamera.y)){
        vec3 start = dir * atmosphereHitsFromCamera.x;
        vec3 end = dir * atmosphereHitsFromCamera.y;
        vec3 sunLost = rayLoseEnergy(starRadiance, getAltitude(start), getAltitude(end), distance(start, end));
        sun *= sunLost;
    } if(hits(atmosphereHitsFromCamera.y)){
        vec3 start = vec3(0.0);
        vec3 end = dir *  atmosphereHitsFromCamera.y;
        vec3 sunLost = rayLoseEnergy(starRadiance, getAltitude(start), getAltitude(end), distance(start, end));
        sun *= sunLost;
    } else {
        sun *= starRadiance;
    }

    float cloudsLambertian = (dot(starDirection, cloudsResult.normal) * 0.5 + 0.5) * 0.5 + 0.5;
    // float fadeModifier = clamp(1.0 - (getForwardHitOrZero(atmosphereHits) / (atmosphereStart * 0.31415)), 0.0, 1.0);
    vec3 resultCloudsRadiance = rayLoseEnergy(starRadiance, rayleighHeight, getAltitude(cloudsResult.distance * dir), atmosphereHitFromClouds) 
        * cloudsColor 
        * 2.0
        * (cloudsLambertian)
        * max(0.0, (cloudsResult.relativeHeight) / (atmosphereHitFromClouds * 0.000001 + 1.0));
    vec3 resultClouds = rayLoseEnergy(resultCloudsRadiance, 
        getAltitude(cloudsResult.distance * dir), 
        getAltitude(dir * getForwardHitOrZero(atmosphereHitsFromCamera)), 
        cloudsResult.distance - (hits(atmosphereHitsFromCamera.x) && hits(atmosphereHitsFromCamera.y) ? getForwardHitOrZero(atmosphereHitsFromCamera) : 0.0)
    );

    vec3 geometryPosition = dir * geometryHit;
    vec3 geometryNormal = normalize(texture(gBufferNormalRGBdistanceA, UV).rgb);
    vec4 geometryColorRoughness = texture(gBufferColorRGBroughnessA, UV).rgba;

    float meshLambertian = max(0.0, dot(geometryNormal, starDirection));
    float meshPhong = pow(meshLambertian, 1.0 + 16.0 * (1.0 - geometryColorRoughness.a * geometryColorRoughness.a));
    
    vec2 atmosphereHitsFromMesh = rsi2X(cloudsResult.distance * dir, starDirection, atmoSphere);
    float atmosphereHitFromMesh = getForwardHitOrZero(atmosphereHitsFromClouds);
    vec3 meshIrradiance = rayLoseEnergy(starRadiance, rayleighHeight, getAltitude(geometryPosition), atmosphereHitFromMesh);
    
    Sphere cloudsMedianSphere = Sphere(bodyCenter, atmosphereStart + (cloudsMaxHeight + cloudsMinHeight) * 0.5);
    float cloudsHit = getForwardHitOrZero(rsi2X(geometryPosition, starDirection, cloudsMedianSphere));
    float meshCloudsShadow = hits(cloudsHit) ? 1.0 - lowResCloudsAtPoint(geometryPosition + cloudsHit * starDirection) : 1.0;
    meshIrradiance *= pow(meshCloudsShadow, 2.0);

    float ao = quickSSAO(0.03);
    vec3 meshAmbient = 0.1 * rayLoseEnergy(starRadiance, rayleighHeight, getAltitude(geometryPosition), atmosphereHitFromMesh)
    * geometryColorRoughness.rgb
    * ((dot(geometryNormal, starDirection) * 0.5 + 0.5) * 0.5 + 0.5) * ao;

    vec3 meshRadiance = meshPhong * geometryColorRoughness.rgb * 
        rayLoseEnergy(meshIrradiance, rayleighHeight, 
        getAltitude(geometryPosition), atmosphereHitFromMesh)
        + max(vec3(0.0), meshAmbient);
    vec3 meshIncoming = rayLoseEnergy(meshRadiance, 
        getAltitude(geometryPosition), 
        getAltitude(dir * getForwardHitOrZero(atmosphereHitsFromCamera)), 
        geometryHit - getForwardHitOrZero(atmosphereHitsFromCamera)
    );

    
    vec3 atmosphereAdditive = rayleighMieResult.rayleigh 
            + sun * (1.0 - step(0.001, geometryHit));
    vec4 atmosphereAlpha = vec4(
            vec3(
                mix(resultClouds, rayleighMieResult.mie.rgb, rayleighMieResult.mie.a)
            ),
            min(1.0, cloudsResult.coverage + rayleighMieResult.mie.a)
    );
    float hitsGeometry = step(0.001, geometryHit);
    vec3 meshFinal = meshIncoming * hitsGeometry;

    outAdditiveRGB = vec4(atmosphereAdditive.rgb, 0.0);
    outAlphaRGBA = vec4(
        mix(meshFinal, 
            atmosphereAlpha.rgb, 
            atmosphereAlpha.a
        ),
        min(1.0, hitsGeometry + atmosphereAlpha.a)
    );
    // outAdditiveRGB = vec4(0.0);
    // outAlphaRGBA = vec4(atmosphereAlpha.rgb, 1.0);
}
