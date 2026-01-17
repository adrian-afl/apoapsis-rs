import * as assert from "node:assert";
import { describe, it } from "node:test";

import { Quaternion, Vector3 } from "@aeroflightlabs/linear-math";

import { dec } from "../../decimalHelpers";
import { DMat3 } from "./DMat3";
import DVec3 from "./DVec3";

describe("mat3d rotations", async () => {
  await it("should rotate around axis", () => {
    const axis = DVec3.fromNumbers(-1.2, 0.4, 0.2).normalized();
    const angle = dec(3.9);
    const matrixDecimal = DMat3.createAxisAngle(axis, angle);
    const vectorDecimal = DVec3.fromNumbers(-1.0, 2.0, 7893.0);
    const vectorThree = Vector3.fromArray(vectorDecimal.asNumbers());
    const quatThree = new Quaternion().setAxisAngle(
      Vector3.fromArray(axis.asNumbers()),
      angle.toNumber(),
    );

    const rotatedDecimal = matrixDecimal.apply(vectorDecimal);
    const rotatedThree = vectorThree.transformQuat(quatThree);

    assert.equal(
      rotatedDecimal.x.toNumber().toFixed(8),
      rotatedThree.x.toFixed(8),
    );
    assert.equal(
      rotatedDecimal.y.toNumber().toFixed(8),
      rotatedThree.y.toFixed(8),
    );
    assert.equal(
      rotatedDecimal.z.toNumber().toFixed(8),
      rotatedThree.z.toFixed(8),
    );
  });

  await it("should rotate around axis 2", () => {
    const axis = DVec3.fromNumbers(0, 1.0, 0.0).normalized();
    const angle = dec(2 * (Math.PI / 180.0));
    const matrixDecimal = DMat3.createAxisAngle(axis, angle);
    const vectorDecimal = DVec3.fromNumbers(1.0, 0.0, 0.0);
    const vectorThree = Vector3.fromArray(vectorDecimal.asNumbers());
    const quatThree = new Quaternion().setAxisAngle(
      Vector3.fromArray(axis.asNumbers()),
      angle.toNumber(),
    );

    const rotatedDecimal = matrixDecimal.apply(vectorDecimal);
    const rotatedThree = vectorThree.transformQuat(quatThree);
    console.log(rotatedThree);

    assert.equal(
      rotatedDecimal.x.toNumber().toFixed(16),
      rotatedThree.x.toFixed(16),
    );
    assert.equal(
      rotatedDecimal.y.toNumber().toFixed(16),
      rotatedThree.y.toFixed(16),
    );
    assert.equal(
      rotatedDecimal.z.toNumber().toFixed(16),
      rotatedThree.z.toFixed(16),
    );
  });
});
