
float flare(vec3 ray){
    float dt = dot(ray, starDirection); // this will return 1 on exact hit, and like 0.9 on almost exact hit
    return step(0.999, dt);
}