import Decimal from "decimal.js";

import { decimalOne, decimalZero } from "../../decimalConstants";
import { dec } from "../../decimalHelpers";
import DVec3 from "./DVec3";

// const math = create(all, {
//   number: "BigNumber", // Default type of number:
//   // 'number' (default), 'BigNumber', or 'Fraction'
//   precision: 64, // Number of significant digits for BigNumbers
//   relTol: 1e-60,
//   absTol: 1e-63,
// });

export class DMat3 {
  public constructor(public readonly data: Decimal[][]) {
    if (data.length !== 3) {
      throw new Error(
        `Expected first dimension to be of length 3, instead got ${data.length}`,
      );
    }
    for (const col of data) {
      if (col.length !== 3) {
        throw new Error(
          `Expected second dimension to be of length 3, instead got ${col.length}`,
        );
      }
    }
  }

  public static identity(): DMat3 {
    const one = decimalOne;
    const zero = decimalZero;
    return new DMat3([
      [one, zero, zero],
      [zero, one, zero],
      [zero, zero, one],
    ]);
  }

  public apply(vector: DVec3): DVec3 {
    return new DVec3(
      dec(0)
        .add(this.data[0][0].mul(vector.x))
        .add(this.data[1][0].mul(vector.y))
        .add(this.data[2][0].mul(vector.z)),

      dec(0)
        .add(this.data[0][1].mul(vector.x))
        .add(this.data[1][1].mul(vector.y))
        .add(this.data[2][1].mul(vector.z)),

      dec(0)
        .add(this.data[0][2].mul(vector.x))
        .add(this.data[1][2].mul(vector.y))
        .add(this.data[2][2].mul(vector.z)),
    );
  }

  public static createAxisAngle(axis: DVec3, angle: Decimal): DMat3 {
    // angle is negated to match the Three JS behavior, no idea why
    // console.log(res.toString());
    const c = angle.negated().cos();
    const s = angle.negated().sin();
    const oneMinusC = dec(1.0).sub(c);
    return new DMat3([
      [
        oneMinusC.mul(axis.x).mul(axis.x).add(c),
        oneMinusC.mul(axis.x).mul(axis.y).sub(axis.z.mul(s)),
        oneMinusC.mul(axis.z).mul(axis.x).add(axis.y.mul(s)),
      ],
      [
        oneMinusC.mul(axis.x).mul(axis.y).add(axis.z.mul(s)),
        oneMinusC.mul(axis.y).mul(axis.y).add(c),
        oneMinusC.mul(axis.y).mul(axis.z).sub(axis.x.mul(s)),
      ],
      [
        oneMinusC.mul(axis.z).mul(axis.x).sub(axis.y.mul(s)),
        oneMinusC.mul(axis.y).mul(axis.z).add(axis.x.mul(s)),
        oneMinusC.mul(axis.z).mul(axis.z).add(c),
      ],
    ]);
  }
}
