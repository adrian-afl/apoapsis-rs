#pragma once

uvec3 executionSize = gl_NumWorkGroups * gl_WorkGroupSize;
ivec2 iUV = ivec2(gl_GlobalInvocationID.xy);
vec2 realFlippedUV = (vec2(gl_GlobalInvocationID.xy) + 0.5) / vec2(executionSize.xy);
vec2 UV = realFlippedUV;//vec2(realFlippedUV.x, 1.0 - realFlippedUV.y);