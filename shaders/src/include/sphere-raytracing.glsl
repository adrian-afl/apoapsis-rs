#pragma once

struct Ray { vec3 o; vec3 d; };
struct Sphere { vec3 pos; float rad; };
float rsi2_simple(in Ray ray, in Sphere sphere)
{
    vec3 oc = ray.o - sphere.pos;
    float b = 2.0 * dot(ray.d, oc);
    return -b - sqrt(b * b - 4.0 * (dot(oc, oc) - sphere.rad*sphere.rad));
}

vec2 rsi2(in Ray ray, in Sphere sphere)
{
    vec3 oc = ray.o - sphere.pos;
    float b = 2.0 * dot(ray.d, oc);
    float c = dot(oc, oc) - sphere.rad*sphere.rad;
    float disc = b * b - 4.0 * c;
    vec2 ex = vec2(-b - sqrt(disc), -b + sqrt(disc))/2.0;
    return vec2(min(ex.x, ex.y), max(ex.x, ex.y));
}

vec2 rsi2X(vec3 eye, vec3 dir, Sphere sphere)
{
    vec3 oc = eye - sphere.pos;
    float b = 2.0 * dot(dir, oc);
    float c = dot(oc, oc) - sphere.rad*sphere.rad;
    float disc = b * b - 4.0 * c;
    vec2 ex = vec2(-b - sqrt(disc), -b + sqrt(disc))/2.0;
    return vec2(min(ex.x, ex.y), max(ex.x, ex.y));
}

vec2 rsi2_intermediate(in Ray ray, in Sphere sphere, float nudge)
{
    vec3 oc = ray.o - sphere.pos + nudge;
    float dst = length(oc);
    float b = 2.0 * dot(ray.d, oc);
    float c = (dst + sphere.rad) * (dst - sphere.rad);
    float disc = b * b - 4.0 * c;
    vec2 ex = vec2(-b - sqrt(disc), -b + sqrt(disc))/2.0;
    return vec2(min(ex.x, ex.y), max(ex.x, ex.y));
}

vec2 rsi2_extra(in Ray ray, in Sphere sphere)
{
    vec3 oc = ray.o - sphere.pos;
    float dst = length(oc);
    float b = 2.0 * dot(ray.d, oc);
    float c = (dst + sphere.rad) * (dst - sphere.rad);
    float disc = sqrt(b * (b - 4.0 * c / b));
    vec2 ex = vec2(-b - disc, -b + disc) / 2.0;
    return vec2(min(ex.x, ex.y), max(ex.x, ex.y));
}

vec2 rsi2_CPUside(in Ray ray, vec3 oc, float dst, float c)
{
    //GPU:
    float dt = 2.0 * dot(ray.d, oc);
    float b = dt * dst;
    // float disc = sqrt(0.01 * (b * b - c)) * sqrt(100.0);
    float disc = sqrt((0.001 * b) * b - c) * sqrt(1000.0);
    // float disc = sqrt(b * b - c);
    vec2 ex = vec2(-b - disc, -b + disc) / 2.0;
    return vec2(min(ex.x, ex.y), max(ex.x, ex.y));
}

float infinity = 99999999999.0;
#define hits(a) (a > 0.0 && a < infinity)

float getForwardHitOrZero(vec2 dualHit){
    if(dualHit.x > 0.0) return dualHit.x;
    if(dualHit.x < 0.0 && dualHit.y > 0.0) return dualHit.y;
    return 0.0;
}

float getCloserHit(float A, float B){
    if(A <= 0.0) return B;
    if(B <= 0.0) return A;
    return min(A, B);
}

float getFurtherHit(float A, float B){
    return max(A, B);
}