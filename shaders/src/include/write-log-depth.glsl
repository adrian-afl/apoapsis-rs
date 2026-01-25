void writeLogDepth(float dist){
//   float C = 0.001;
//   float w = length(dist);
//   float Far = 637800000.0;
//   gl_FragDepth = min(1.0, log(C * w + 1.0) / log(C * Far + 1.0));
  gl_FragDepth = dist / 10000.0;
}
