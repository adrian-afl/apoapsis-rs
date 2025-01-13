
float gw3d_hash(float p){
    return fract(4768.1232345456 * sin(p));
}

vec2 gw3d_wavedx(vec3 position, vec3 direction, float speed, float frequency, float timeshift) {
    float x = dot(direction, position) * frequency + timeshift * speed;
    float wave = exp(sin(x) - 1.0);
    float dx = wave * cos(x);
    return vec2(wave, -dx);
}

float gw3d_seedWaves = 0.0;
vec3 gw3d_randWaves(){
    float x = gw3d_hash(gw3d_seedWaves);
    gw3d_seedWaves += 1.0;
    float y = gw3d_hash(gw3d_seedWaves);
    gw3d_seedWaves += 1.0;
    float z = gw3d_hash(gw3d_seedWaves);
    gw3d_seedWaves += 1.0;
    return vec3(x,y,z) * 2.0 - 1.0;
}

float getwaves3d(
    int iterations,
    vec3 position, 
    float dragmult, 
    float timeshift,
    float weightCoef,
    float phaseCoef
){
    float phase = 6.0;
    float speed = 2.0;
    float weight = 1.0;
    float w = 0.0;
    float ws = 0.0;
    for(int i=0;i<iterations;i++){
        vec3 p = gw3d_randWaves() * 1.21;
        vec2 res = gw3d_wavedx(position, p, speed, phase, 0.0 + timeshift);
        position -= normalize(position - p) * res.y * weight * dragmult * 0.01;
        w += res.x * weight;
        ws += weight;
        weight = mix(weight, 0.0, weightCoef);
        phase *= phaseCoef;
        speed *= 1.02;
    }
    return w / ws;
}