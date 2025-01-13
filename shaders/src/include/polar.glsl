#pragma once

vec2 xyzToPolar(vec3 xyz){
    float theta = atan(-xyz.x, -xyz.z);
    float phi = acos(-xyz.y);
    vec2 result = vec2(theta, phi) / vec2(2.0 * 3.1415,  3.1415);
    result.x = fract(result.x);
    return result;
}

// vec2 xyzToPolar(vec3 xyz) {
//   float theta = atan(xyz.y, xyz.x);
//   float phi = acos(clamp(xyz.z, -1.0, 1.0));
//   return vec2(theta, phi) / vec2(2.0 * 3.1415, 3.1415);
// }
//
//vec2 xyzToPolar(vec3 xyz) {
//  float theta = atan(xyz.y, xyz.x);
//  float psi = atan(xyz.z, sqrt(xyz.x * xyz.x + xyz.y * xyz.y));
//  return vec2(theta, psi) / vec2(2.0 * 3.1415, 2.0 * 3.1415);
//}

//vec2 xyzToPolar(vec3 xyz) {
//  float radius = sqrt(xyz.x * xyz.x + xyz.y * xyz.y + xyz.z * xyz.z);
//  float theta = atan(xyz.y, xyz.x);
//  float phi = acos(xyz.z / radius);
//  return vec2(theta, phi) / vec2(2.0 * 3.1415, 3.1415);
//}

vec3 polarToXyz(vec2 xy) {
  xy *= vec2(2.0 * 3.1415, 3.1415);
  float z = cos(xy.y);
  float x = cos(xy.x) * sin(xy.y);
  float y = sin(xy.x) * sin(xy.y);
  return normalize(vec3(x, y, z));
}
