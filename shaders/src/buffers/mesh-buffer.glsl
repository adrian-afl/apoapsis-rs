layout(set = MESH_BUFFER_SET, binding = MESH_BUFFER_BINDING) uniform meshDataBuffer {
    mat4 perspectiveMatrix;
    mat4 viewMatrix;
    mat4 modelMatrix;

    vec4 color_zero;

    uint useColorTexture;
    uint useRoughnessTexture;
    uint useMetalnessTexture;
    uint useEmissionTexture;

    uint useNormalTexture;
    uint useBumpTexture;
    float colorTextureScale;
    float roughnessTextureScale;

    float roughness;
    float metalness;
    float metalnessTextureScale;
    float emissionTextureScale;

    vec4 emission_zero;

    float normalTextureScale;
    float bumpTextureScale;
} meshData;

mat4 perspectiveMatrix = meshData.perspectiveMatrix;
mat4 viewMatrix = meshData.viewMatrix;
mat4 modelMatrix = meshData.modelMatrix;

vec3 color = meshData.color_zero.rgb;
uint useColorTexture = meshData.useColorTexture;
float colorTextureScale = meshData.colorTextureScale;

float roughness = meshData.roughness;
uint useRoughnessTexture = meshData.useRoughnessTexture;
float roughnessTextureScale = meshData.roughnessTextureScale;

float metalness = meshData.metalness;
uint useMetalnessTexture = meshData.useMetalnessTexture;
float metalnessTextureScale = meshData.metalnessTextureScale;

vec3 emission = meshData.emission_zero.rgb;
uint useEmissionTexture = meshData.useEmissionTexture;
float emissionTextureScale = meshData.emissionTextureScale;

uint useNormalTexture = meshData.useNormalTexture;
float normalTextureScale = meshData.normalTextureScale;

uint useBumpTexture = meshData.useBumpTexture;
float bumpTextureScale = meshData.bumpTextureScale;