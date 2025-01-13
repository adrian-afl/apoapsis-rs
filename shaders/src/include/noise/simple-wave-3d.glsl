vec3 _simple_getwaves_directions[] = vec3[](
  vec3(0.7900, 0.5533, -0.2640),
  vec3(-0.9075, 0.1099, 0.4055),
  vec3(0.7029, -0.5427, 0.4598),
  vec3(-0.1990, -0.7706, -0.6054),
  vec3(-0.8966, 0.2679, -0.3526),
  vec3(-0.1806, 0.4303, -0.8844),
  vec3(-0.0061, 0.8324, -0.5542),
  vec3(0.5143, -0.6805, 0.5219),
  vec3(-0.5450, 0.7928, 0.2727),
  vec3(0.5874, -0.7927, -0.1632),
  vec3(0.4356, -0.1174, 0.8925),
  vec3(-0.2174, 0.1649, -0.9621),
  vec3(-0.5134, -0.0137, -0.8581),
  vec3(-0.3361, -0.1214, 0.9340),
  vec3(0.6320, -0.4675, -0.6181),
  vec3(0.2561, 0.1685, -0.9519),
  vec3(0.7354, 0.6738, 0.0716),
  vec3(-0.0798, 0.9033, -0.4215),
  vec3(-0.1344, -0.6286, -0.7660),
  vec3(0.4724, 0.6836, 0.5564),
  vec3(-0.5242, -0.6188, 0.5851),
  vec3(0.0763, 0.0929, -0.9927),
  vec3(-0.9852, -0.1562, -0.0712),
  vec3(-0.2936, -0.7704, 0.5660),
  vec3(-0.4166, -0.7558, 0.5051),
  vec3(0.5641, -0.1422, 0.8134),
  vec3(-0.1560, 0.3815, -0.9111)
);

float simplegetwaves(vec3 position, float dragmult){
    int iters = _simple_getwaves_directions.length();
    float result = 0.0;
    for(int i=0;i<iters;i++){
        float x = dot(_simple_getwaves_directions[i], position);
      
        float wave = exp(sin(x) - 1.0);
        float dx = wave * cos(x);
      
        result += wave;
        position += _simple_getwaves_directions[i] * dx * dragmult;
    }
    return result / float(iters);
}