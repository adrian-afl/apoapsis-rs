float oct(vec3 p){
    return fract(4768.1232345456 * sin((p.x+p.y*43.0+p.z*137.0)));
}
float oct_tiled(vec3 p){
    p = mod(p, 4768.1232345456);
    return fract(4768.1232345456 * sin((p.x+p.y*43.0+p.z*137.0)));
}
float achnoise(vec3 x){
    vec3 p = floor(x);
    vec3 fr = smoothstep(0.0, 1.0, fract(x));
    vec3 LBZ = p + vec3(0.0, 0.0, 0.0);
    vec3 LTZ = p + vec3(0.0, 1.0, 0.0);
    vec3 RBZ = p + vec3(1.0, 0.0, 0.0);
    vec3 RTZ = p + vec3(1.0, 1.0, 0.0);

    vec3 LBF = p + vec3(0.0, 0.0, 1.0);
    vec3 LTF = p + vec3(0.0, 1.0, 1.0);
    vec3 RBF = p + vec3(1.0, 0.0, 1.0);
    vec3 RTF = p + vec3(1.0, 1.0, 1.0);

    float l0candidate1 = oct(LBZ);
    float l0candidate2 = oct(RBZ);
    float l0candidate3 = oct(LTZ);
    float l0candidate4 = oct(RTZ);

    float l0candidate5 = oct(LBF);
    float l0candidate6 = oct(RBF);
    float l0candidate7 = oct(LTF);
    float l0candidate8 = oct(RTF);

    float l1candidate1 = mix(l0candidate1, l0candidate2, fr[0]);
    float l1candidate2 = mix(l0candidate3, l0candidate4, fr[0]);
    float l1candidate3 = mix(l0candidate5, l0candidate6, fr[0]);
    float l1candidate4 = mix(l0candidate7, l0candidate8, fr[0]);


    float l2candidate1 = mix(l1candidate1, l1candidate2, fr[1]);
    float l2candidate2 = mix(l1candidate3, l1candidate4, fr[1]);


    float l3candidate1 = mix(l2candidate1, l2candidate2, fr[2]);

    return l3candidate1;
}
float achnoise_tiled(vec3 x){
    vec3 p = floor(x);
    vec3 fr = smoothstep(0.0, 1.0, fract(x));
    vec3 LBZ = p + vec3(0.0, 0.0, 0.0);
    vec3 LTZ = p + vec3(0.0, 1.0, 0.0);
    vec3 RBZ = p + vec3(1.0, 0.0, 0.0);
    vec3 RTZ = p + vec3(1.0, 1.0, 0.0);

    vec3 LBF = p + vec3(0.0, 0.0, 1.0);
    vec3 LTF = p + vec3(0.0, 1.0, 1.0);
    vec3 RBF = p + vec3(1.0, 0.0, 1.0);
    vec3 RTF = p + vec3(1.0, 1.0, 1.0);

    float l0candidate1 = oct_tiled(LBZ);
    float l0candidate2 = oct_tiled(RBZ);
    float l0candidate3 = oct_tiled(LTZ);
    float l0candidate4 = oct_tiled(RTZ);

    float l0candidate5 = oct_tiled(LBF);
    float l0candidate6 = oct_tiled(RBF);
    float l0candidate7 = oct_tiled(LTF);
    float l0candidate8 = oct_tiled(RTF);

    float l1candidate1 = mix(l0candidate1, l0candidate2, fr[0]);
    float l1candidate2 = mix(l0candidate3, l0candidate4, fr[0]);
    float l1candidate3 = mix(l0candidate5, l0candidate6, fr[0]);
    float l1candidate4 = mix(l0candidate7, l0candidate8, fr[0]);


    float l2candidate1 = mix(l1candidate1, l1candidate2, fr[1]);
    float l2candidate2 = mix(l1candidate3, l1candidate4, fr[1]);


    float l3candidate1 = mix(l2candidate1, l2candidate2, fr[2]);

    return l3candidate1;
}

float supernoise3D(vec3 p){

	float a =  achnoise(p);
	float b =  achnoise(p + 2.5);
	return (a + b) * 0.5;
}

float supernoise3Dtiled(vec3 p){

	float a =  achnoise_tiled(p);
	float b =  achnoise_tiled(p + 2.5);
	return (a + b) * 0.5;
}

float supernoise3DMUL(vec3 p){

	float a =  achnoise(p);
	float b =  achnoise(p + 2.5);
	return a * b;
}