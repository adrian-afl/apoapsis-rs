#version 460
#extension GL_ARB_separate_shader_objects : enable

uniform sampler2D additiveRGB;
uniform sampler2D alphaRGBA;
uniform sampler2D backbuffer;

in vec2 UV;

layout (location = 0) out vec4 outColor;

void main() {
    vec4 bbdata = texture(backbuffer, UV);
    vec4 additive = texture(additiveRGB, UV).rgba;
    vec4 alpha = texture(alphaRGBA, UV).rgba;

    vec3 color = bbdata.rgb * (1.0 - alpha.a) + additive.rgb + alpha.rgb;

    outColor = vec4(color, 1.0);
}
