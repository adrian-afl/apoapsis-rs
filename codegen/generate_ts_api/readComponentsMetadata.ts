import * as fs from "fs";

export function readComponentsMetadata() {
  const componentEnumMacroRegex =
    /\([ \n\r]*?([a-z_]+?),[ \n\r]*?([A-z]+?),[ \n\r]*?([A-z]+?),[ \n\r]*?(Option|Marker|Vector)[ \n\r]*?\)/gms;

  const componentsDefFile = fs
    .readFileSync("../../packages/ecs/src/component_trait.rs")
    .toString("utf-8");

  const componentsDefFileLines = componentsDefFile.split("\n");

  return [...componentsDefFile.matchAll(componentEnumMacroRegex)].map((x) => ({
    snake: x[1],
    short: x[2],
    full: x[3],
    type: x[4] as "Vector" | "Option" | "Marker",
    importRs: componentsDefFileLines
      .find((y) => y.startsWith("use crate::") && y.includes(x[3]))
      ?.replace("use crate::", "use ecs::"),
    importTs: `import { ${x[3]} } from "./types/${x[3]}.js";`,
  }));
}
