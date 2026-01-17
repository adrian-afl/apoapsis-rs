import Decimal from "decimal.js";

import { Vector3 } from "@aeroflightlabs/linear-math";

import { decimalZero } from "../../decimalConstants";
import { DecimalVector3d } from "../../../../generated/types/DecimalVector3d";

class DVec3 {
  private static vecZero = new DVec3(decimalZero, decimalZero, decimalZero);

  public constructor(
    public readonly x: Decimal,
    public readonly y: Decimal,
    public readonly z: Decimal,
  ) {}

  public static fromNumbers(x: number, y: number, z: number): DVec3 {
    return new this(new Decimal(x), new Decimal(y), new Decimal(z));
  }

  public static fromFloat64Vector(vector: Vector3): DVec3 {
    return new this(
      new Decimal(vector.x),
      new Decimal(vector.y),
      new Decimal(vector.z),
    );
  }

  public static fromStrings(x: string, y: string, z: string): DVec3 {
    return new this(new Decimal(x), new Decimal(y), new Decimal(z));
  }

  public static fromNumbersArray(array: [number, number, number]): DVec3 {
    return new this(
      new Decimal(array[0]),
      new Decimal(array[1]),
      new Decimal(array[2]),
    );
  }

  public static fromDecimalVector3d(input: DecimalVector3d): DVec3 {
    return new this(
      new Decimal(input.x),
      new Decimal(input.y),
      new Decimal(input.z),
    );
  }

  public toDecimalVector3d(): DecimalVector3d {
    const strings = this.toStringArray();
    return {
      x: strings[0],
      y: strings[1],
      z: strings[2],
    };
  }

  public static zero(): DVec3 {
    return DVec3.vecZero;
  }

  public toString(decimalPoints = 40): string {
    return `{ x: ${this.x.toFixed(decimalPoints)}, y: ${this.y.toFixed(decimalPoints)}, z: ${this.z.toFixed(decimalPoints)} }`;
  }

  public toStringArray(): [string, string, string] {
    return [this.x.toFixed(40), this.y.toFixed(40), this.z.toFixed(40)];
  }

  public toFloat64Vector(): Vector3 {
    return new Vector3(this.x.toNumber(), this.y.toNumber(), this.z.toNumber());
  }

  public length(): Decimal {
    const sumOfCubes = this.x
      .mul(this.x)
      .add(this.y.mul(this.y))
      .add(this.z.mul(this.z));
    return sumOfCubes.sqrt();
  }

  public lengthSquared(): Decimal {
    return this.x.mul(this.x).add(this.y.mul(this.y)).add(this.z.mul(this.z));
  }

  public shuffle(schema: string): DVec3 {
    if (schema.length !== 3) {
      throw new Error(`Invalid shuffle ${schema}`);
    }
    const parts = schema.split("").map((l) => {
      if (l !== "x" && l !== "y" && l !== "z") {
        throw new Error(`Invalid shuffle ${schema}`);
      }
      let result = this.x;
      if (l === "y") {
        result = this.y;
      }
      if (l === "z") {
        result = this.z;
      }
      return result;
    });
    return new DVec3(parts[0], parts[1], parts[2]);
  }

  public distanceTo(input: DVec3): Decimal {
    const difference = input.subVec3(this);
    const sumOfCubes = difference.x
      .mul(difference.x)
      .add(difference.y.mul(difference.y))
      .add(difference.z.mul(difference.z));
    return sumOfCubes.sqrt();
  }

  public normalized(): DVec3 {
    const length = this.length();
    if (length.eq(decimalZero)) {
      return DVec3.zero();
    }
    return new DVec3(
      this.x.div(length),
      this.y.div(length),
      this.z.div(length),
    );
  }

  public asNumbers(): [number, number, number] {
    return [this.x.toNumber(), this.y.toNumber(), this.z.toNumber()];
  }

  public dot(input: DVec3): Decimal {
    return this.x
      .mul(input.x)
      .add(this.y.mul(input.y))
      .add(this.z.mul(input.z));
  }

  public cross(input: DVec3): DVec3 {
    const b = input;

    const ax = this.x;
    const ay = this.y;
    const az = this.z;
    const bx = b.x;
    const by = b.y;
    const bz = b.z;

    const x = ay.mul(bz).sub(az.mul(by));
    const y = az.mul(bx).sub(ax.mul(bz));
    const z = ax.mul(by).sub(ay.mul(bx));

    return new DVec3(x, y, z);
  }

  public addVec3(input: DVec3): DVec3 {
    return new DVec3(
      this.x.add(input.x),
      this.y.add(input.y),
      this.z.add(input.z),
    );
  }

  public subVec3(input: DVec3): DVec3 {
    return new DVec3(
      this.x.sub(input.x),
      this.y.sub(input.y),
      this.z.sub(input.z),
    );
  }

  public mulVec3(input: DVec3): DVec3 {
    return new DVec3(
      this.x.mul(input.x),
      this.y.mul(input.y),
      this.z.mul(input.z),
    );
  }

  public divVec3(input: DVec3): DVec3 {
    return new DVec3(
      this.x.div(input.x),
      this.y.div(input.y),
      this.z.div(input.z),
    );
  }

  public addScalar(input: Decimal): DVec3 {
    return new DVec3(this.x.add(input), this.y.add(input), this.z.add(input));
  }

  public subScalar(input: Decimal): DVec3 {
    return new DVec3(this.x.sub(input), this.y.sub(input), this.z.sub(input));
  }

  public mulScalar(input: Decimal): DVec3 {
    return new DVec3(this.x.mul(input), this.y.mul(input), this.z.mul(input));
  }

  public divScalar(input: Decimal): DVec3 {
    return new DVec3(this.x.div(input), this.y.div(input), this.z.div(input));
  }
}

export default DVec3;
