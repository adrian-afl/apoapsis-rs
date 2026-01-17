import { Quaternion, Vector3 } from "@aeroflightlabs/linear-math";

// TODO maybe move this to aflmath
export function getQuatDirections(q: Quaternion): {
  up: Vector3;
  down: Vector3;
  left: Vector3;
  right: Vector3;
  forwards: Vector3;
  backwards: Vector3;
} {
  const forwards = new Vector3(0, 0, -1).transformQuat(q);
  const backwards = new Vector3(0, 0, 1).transformQuat(q);
  const up = new Vector3(0, 1, 0).transformQuat(q);
  const down = new Vector3(0, -1, 0).transformQuat(q);
  const left = new Vector3(-1, 0, 0).transformQuat(q);
  const right = new Vector3(1, 0, 0).transformQuat(q);
  return {
    up,
    down,
    left,
    right,
    forwards,
    backwards,
  };
}
