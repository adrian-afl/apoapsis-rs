vec3 rayleighColor = vec3(0.274, 0.427, 0.862);
vec3 invertRayleighColor = vec3(1.0 - 0.274, 1.0 - 0.427, 1.0 - 0.862);

float rayleighDensityAtAltitude(float altitude){
    float x = 1.0 - clamp((altitude / rayleighHeight), 0.0, 1.0);
    float xp3 = x*x*x;
    return rayleighDensity * xp3;
}

float rayleighAtAltitude(float altitude, float shadow){
    return shadow * rayleighDensityAtAltitude(altitude);
}

float mieDensityAtAltitude(float altitude){
    float x = 1.0 - clamp((altitude / mieHeight), 0.0, 1.0);
    float xp3 = x*x*x;
    return mieDensity * xp3;
}

vec4 mieAtAltitude(float altitude, vec3 dir, float shadow){
    float density = mieDensityAtAltitude(altitude);
    return vec4(shadow * mieColor * density, density);
}

vec3 rayLoseEnergy(vec3 initialColor, float sourceAltitude, float receiveAltitude, float dist){
    if(dist <= 0.0) return initialColor;
    float scatterSecondary = (dist / 500000.0) 
        * rayleighDensityAtAltitude(mix(receiveAltitude, sourceAltitude, 0.5));
    return max(vec3(0.0), initialColor - rayleighColor * scatterSecondary);
}

// vec3 getLightComingToAPoint()

RayleighMieResult raymarchRayleighMie(vec3 start, vec3 end, float cloudsDistance, float cloudsCoverage) {
  float rd = fract(rand2d(UV) + elapsed);
  vec3 relative = (end - start);
  vec3 dir = normalize(relative);
  float dist = length(relative);
  float stepsF = max(98.0, min(dist * 0.0001, 2232.0));
  int stepsI = int(stepsF);
  float stepsize = 1.0/stepsF;
  float worldStepSize = dist * stepsize;
//   vec3 pos = start + worldStepSize * rd;

  vec3 resultRayleigh = vec3(0.0);
  vec4 resultMie = vec4(0.0, 0.0, 0.0, 1.0);

  Sphere planetSphere = Sphere(bodyCenter, atmosphereStart);
  Sphere rayleighSphere = Sphere(bodyCenter, atmosphereStart + rayleighHeight);

  /*
  iterate over the ray

  */

  float expDirectional = exp((dot(starDirection, dir) * 0.5 + 0.5) * 10.0) * 0.001;

  float startAltitude = getAltitude(start);

  Sphere cloudsMedianSphere = Sphere(bodyCenter, atmosphereStart + (cloudsMaxHeight + cloudsMinHeight) * 0.5);

  float currentDist = rd * 100.0;
  float limitDist = distance(start, end);

  int limit = 1024;

  while(limit-->0) {
    vec3 pos = start + dir * currentDist;
    float stepsize = mix(1.0, 10000.0, min(1.0, currentDist / 1000000.0));
    float altitude = getAltitude(pos);

    vec2 planetHits = rsi2X(pos, starDirection, planetSphere);
    float atmoHits = getForwardHitOrZero(rsi2X(pos, starDirection, rayleighSphere));
    float cloudsHit = getForwardHitOrZero(rsi2X(pos, starDirection, cloudsMedianSphere));
    float shadow = 1.0 - step(0.1, planetHits.x);

    float godRayShadow = hits(cloudsHit) ? 1.0 - lowResCloudsAtPoint(pos + cloudsHit * starDirection) : 1.0;
    shadow *= pow(godRayShadow, 3.0);

    vec3 lightGettingScattered = rayLoseEnergy(starIrradiance, rayleighHeight, altitude, atmoHits);
    vec3 lightIncomingToCamera = rayleighColor 
                    * rayLoseEnergy(lightGettingScattered, altitude, startAltitude, distance(start, pos));

    float isBeforeClouds = mix(1.0, (1.0 - step(cloudsDistance, length(pos))), cloudsCoverage);

    if(isBeforeClouds <= 0.0){
        break;
    }
    if(resultMie.a <= 0.0){
        break;
    }

    // exp here is unphysical but looks nicer
    resultRayleigh += lightIncomingToCamera 
    * rayleighAtAltitude(altitude, shadow) 
    * isBeforeClouds
    * (resultMie.a)
    * (expDirectional * 0.1 + 0.9)
    * stepsize;
    
    vec3 mieLightIncomingToCamera = rayLoseEnergy(lightGettingScattered, altitude, startAltitude, distance(start, pos));

    vec4 mieModifier = mieAtAltitude(altitude, dir, shadow) 
        * isBeforeClouds
        * 0.001 * stepsize;

    resultMie.xyz += mieLightIncomingToCamera
    * mieModifier.xyz
    * (expDirectional * 0.5 + 0.5)
    * (resultMie.a);

    resultMie.a *= 1.0 - mieModifier.a;

    currentDist += stepsize;
    
    if(currentDist > limitDist){
        break;
    }
  }

  resultRayleigh *= 0.00001;
  resultMie.a = 1.0 - clamp(resultMie.a, 0.0, 1.0);
  return RayleighMieResult(resultRayleigh, resultMie); 
}

RayleighMieResult rayleighMie(vec3 dir, float geometryHit, CloudsResult cloudsResult){
    Sphere atmoSphere = Sphere(bodyCenter, atmosphereStart + max(rayleighHeight, mieHeight));
    vec2 atmosphereHits = rsi2X(vec3(0.0), dir, atmoSphere);

    if(!hits(atmosphereHits.x) && !hits(atmosphereHits.y)){
        return RayleighMieResult(vec3(0.0), vec4(0.0));
    }

    vec3 start = vec3(0.0);
    vec3 end = vec3(0.0);

    if(hits(atmosphereHits.x) && hits(atmosphereHits.y)){
        // outside of atmosphere - in space
        start = dir * atmosphereHits.x;
        end = dir * (geometryHit > 0.0 ? min(geometryHit, atmosphereHits.y): atmosphereHits.y);
    } else {
        // inside the atmosphere
        end = dir * (geometryHit > 0.0 ? min(geometryHit, atmosphereHits.y): atmosphereHits.y);
    }

    if(cloudsResult.distance == 0.0 && cloudsResult.coverage == 0.0 ){
       cloudsResult.distance = length(end); // discard fix
    }
    // cloudsData.g = mix(length(end), cloudsData.g, cloudsData.a);
    return raymarchRayleighMie(start, end, cloudsResult.distance, cloudsResult.coverage);
}