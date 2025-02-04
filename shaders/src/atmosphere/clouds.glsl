#include "include/frustum-cone.glsl"
#include "include/polar.glsl"
#include "include/sphere-raytracing.glsl"
#include "include/texture/3d.glsl"
#include "include/texture/linear.glsl"

// this function probes the clouds density at a point
// returns XY
// X = coverage of clouds at this point, 
// Y = cloud color at this point, basically incoming radiance
vec2 cloudsDensity3D(vec3 pos, bool lowRes){
    float h = getAltitude(pos);
    vec3 spherespace = mat3(rotationMatrix) * (pos - bodyCenter);
    float dist = length(pos);
    vec3 n = normalize(spherespace);
   
    float measurement = (cloudsMaxHeight - cloudsMinHeight) * 0.5;
    float mediana = (cloudsMaxHeight + cloudsMinHeight) * 0.5;
    float mlt = (( 1.0 - (abs( h - mediana ) / measurement )));

    int iterations = clamp(int((1.0 / (1.0 + length(pos) * 0.00000001)) * 4.0), 0, 4);
    vec2 coords = xyzToPolar(normalize(spherespace));

     float lowFreq = textureLinear(cloudsLowFreqTextureDensityR, coords * 1.0).r * 0.6;

    float density = lowFreq;

    if(dist < 5000000.0){
        density += 0.05 * (texture(cloudsHighFreqTextureDensityR, spherespace * 0.000001 + elapsed * 0.001).r * 2.0 - 1.0);
    }

    if(dist < 2000000.0){
        density += 0.02 * (texture(cloudsHighFreqTextureDensityR, spherespace * 0.000008 + elapsed * 0.001).r * 2.0 - 1.0);
    }

    if(!lowRes){
        if(dist < 500000.0){
            density += 0.01 * (texture(cloudsHighFreqTextureDensityR, spherespace * 0.000019 + elapsed * 0.001).r * 2.0 - 1.0);
        }
        
        if(dist < 100000.0){
            density += 0.005 * (texture(cloudsHighFreqTextureDensityR, spherespace * 0.000147 + elapsed * 0.001).r * 2.0 - 1.0);
        }

        if(dist < 10000.0){
            density += 0.002 * (texture(cloudsHighFreqTextureDensityR, spherespace * 0.001147 + elapsed * 0.001).r * 2.0 - 1.0);
        }
    }

    float scattering = (h - cloudsMinHeight) / (cloudsMaxHeight - cloudsMinHeight);
    
    return vec2(density * mlt, pow(scattering * 1.5, 4.0));
}

#define COVERAGE_START 0.1
#define COVERAGE_END 0.3

float lowResCloudsAtPoint(vec3 pos) {
    vec2 density = cloudsDensity3D(pos, true);
    return smoothstep(COVERAGE_START, COVERAGE_END, clamp(density.x, 0.0, 1.0));
}

CloudsResult raymarchClouds(vec3 start, vec3 end, vec3 dir) {  
  float rd = fract(rand2d(UV) + elapsed);

  float coverageinv = 1.0;

//   float stepSizeClouds = 1.0/128.0;
//   float worldStepSizeClouds = distance(start, end) * stepSizeClouds;

  float avgDistance = 0.0;//length(mix(start, end, 0.5));
  float avgDistanceWeight = 0.0000001;

  float illumination = 0.0;
  float illuminationWeight = 0.0000001;

  float currentDist = 0.0;
  float limitDist = distance(start, end);

  int limit = 1024;

  while(limit-->0) {
    vec3 pos = start + dir * currentDist;

    vec2 density = cloudsDensity3D(pos, false);
    float clouds = smoothstep(COVERAGE_START, COVERAGE_END, clamp(density.x, 0.0, 1.0));

    illumination += coverageinv * density.y * clouds;
    
    avgDistance += coverageinv * currentDist * clouds;

    illuminationWeight += coverageinv * clouds;
    avgDistanceWeight += coverageinv * clouds;

    //clouds = mix(clouds, 1.0, min(1.0, worldStepSizeClouds * 0.01));

    // add coverage by subtracting from the inverted coverage, and subtract a bit more for fog rendering
    coverageinv = max(0.0, 
        coverageinv - clouds
    );

    currentDist += mix(1.0, 10000.0, min(1.0, currentDist / 1000000.0));
    
    if(coverageinv == 0.0 || currentDist > limitDist){
        break;
    }
  }

  avgDistance = avgDistanceWeight > 0.0 ? avgDistance / avgDistanceWeight : 0.0;
  
//  avgDistance += (1.0 - step(0.01, avgDistance)) * length(mix(start, end, 0.5));

  illumination /= illuminationWeight;

  vec3 rouglyhHitPos = start + dir * avgDistance;
  float density000 = cloudsDensity3D(rouglyhHitPos, true).x;
  float density100 = cloudsDensity3D(rouglyhHitPos + vec3(1000.0, 0.0, 0.0), true).x;
  float density010 = cloudsDensity3D(rouglyhHitPos + vec3(0.0, 1000.0, 0.0), true).x;
  float density001 = cloudsDensity3D(rouglyhHitPos + vec3(0.0, 0.0, 1000.0), true).x;

  vec3 normal = normalize(vec3(
    density000 - density100,
    density000 - density010,
    density000 - density001
  ));

  float coverage = 1.0 - clamp(coverageinv, 0.0, 1.0);  
  return CloudsResult(coverage, length(start) + avgDistance, illumination, normal);
}

CloudsResult clouds(vec3 dir, float geometryDistance) {
  vec3 start = vec3(0.0);
    
  Sphere cloudsMinSphere = Sphere(bodyCenter, atmosphereStart + cloudsMinHeight);
  Sphere cloudsMaxSphere = Sphere(bodyCenter, atmosphereStart + cloudsMaxHeight);

  vec2 cloudsMinHeightHit = rsi2X(start, dir, cloudsMinSphere);
  vec2 cloudsMaxHeightHit = rsi2X(start, dir, cloudsMaxSphere);

  float cloudsMinFHit = getForwardHitOrZero(cloudsMinHeightHit);
  float cloudsMaxFHit = getForwardHitOrZero(cloudsMaxHeightHit);

  if(!hits(cloudsMinFHit) && !hits(cloudsMaxFHit)){
    return CloudsResult(0.0, 0.0, 0.0, vec3(0.0));
  }

  float closerCloudsHit = getCloserHit(cloudsMinFHit, cloudsMaxFHit);
  float furtherCloudsHit = getFurtherHit(cloudsMinFHit, cloudsMaxFHit);
  
  vec3 end = dir * closerCloudsHit;

    float startAltitude = getAltitude(start);
    if(startAltitude > cloudsMinHeight && startAltitude < cloudsMaxHeight){
        end = dir * getCloserHit(geometryDistance, max(cloudsMinFHit, cloudsMaxFHit));
    } else if(startAltitude < cloudsMinHeight) {
        // Under the clouds
        start = dir * cloudsMinFHit;
        end = dir * cloudsMaxFHit;
    } else {
        // Over the clouds
        if(!hits(cloudsMinFHit)){
            start = dir * cloudsMaxHeightHit.x;
            end = dir * getCloserHit(geometryDistance, cloudsMaxHeightHit.y);
        } else {
            start = dir * closerCloudsHit;
            end = dir * getCloserHit(geometryDistance, furtherCloudsHit);
        }
    }

    if(hits(geometryDistance) && geometryDistance < length(start)){
       return CloudsResult(0.0, 0.0, 0.0, vec3(0.0));
    }
    
    return raymarchClouds(start, end, dir);
}
