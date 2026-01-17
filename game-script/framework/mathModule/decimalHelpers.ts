import Decimal from "decimal.js";

Decimal.set({ precision: 32, rounding: 8 });

export function dec(value: string | number): Decimal {
  if (typeof value === "string") {
    return new Decimal(value);
  } else {
    return new Decimal(value);
  }
}
