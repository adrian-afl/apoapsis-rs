#version 460
#extension GL_ARB_separate_shader_objects : enable

uniform int debugTextureIndex;

uniform sampler2D celestialResultTexture;

uniform float exposure;

uniform sampler2D debugTexture1;
uniform sampler2D debugTexture2;
uniform sampler2D debugTexture3;
uniform sampler2D debugTexture4;

in vec2 UV;

out vec4 outColor;

vec3 aces_tonemap(vec3 color){	
	mat3 m1 = mat3(
        0.59719, 0.07600, 0.02840,
        0.35458, 0.90834, 0.13383,
        0.04823, 0.01566, 0.83777
	);
	mat3 m2 = mat3(
        1.60475, -0.10208, -0.00327,
        -0.53108,  1.10813, -0.07276,
        -0.07367, -0.00605,  1.07602
	);
	vec3 v = m1 * color;    
	vec3 a = v * (v + 0.0245786) - 0.000090537;
	vec3 b = v * (0.983729 * v + 0.4329510) + 0.238081;
	return pow(clamp(m2 * (a / b), 0.0, 1.0), vec3(1.0 / 2.2));	
}

void main() {
    if(debugTextureIndex == 1) {outColor = vec4(texture(debugTexture1, UV).rgb, 1.0); return; }
    if(debugTextureIndex == 2) {outColor = vec4(texture(debugTexture2, UV).rgb, 1.0); return; }
    if(debugTextureIndex == 3) {outColor = vec4(texture(debugTexture3, UV).rgb, 1.0); return; }
    if(debugTextureIndex == 4) {outColor = vec4(texture(debugTexture4, UV).rgb, 1.0); return; }
    
    vec4 celestialResult = texture(celestialResultTexture, UV);

    outColor = vec4(aces_tonemap(celestialResult.rgb * exposure), 1.0);
}
