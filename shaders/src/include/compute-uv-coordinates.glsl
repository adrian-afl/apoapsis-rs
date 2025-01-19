#pragma once

uvec3 executionSize = gl_NumWorkGroups * gl_WorkGroupSize;
ivec2 inviUV = ivec2(gl_GlobalInvocationID.x, executionSize.y - gl_GlobalInvocationID.y - 1);
ivec2 iUV = ivec2(gl_GlobalInvocationID.xy);

vec2 UV = (vec2(gl_GlobalInvocationID.xy) + 0.5) / vec2(executionSize.xy);