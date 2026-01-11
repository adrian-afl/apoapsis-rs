export function retfun<T>(fn: () => T) {
  return fn();
}
