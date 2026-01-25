import * as fs from "node:fs";
import { parseArgs } from "node:util";

import { loadObjFileAsSingleGeometry } from "./objLoader/loadObjFile";

const { values } = parseArgs({
  options: {
    input: { type: "string" },
    output: { type: "string" },
    format: { type: "string" },
    help: { type: "boolean" },
  },
  strict: true,
});

if (values.help) {
  console.log(
    "Hello to this amazing utility that turns normal models into raw mess!!",
  );
  console.log("Usage:");
  console.log(
    `  this-converter.ts --input dingus.obj --output dingus.raw --format PNUT`,
  );
  console.log("  The format is combination of following options:");
  console.log(
    "    P - position (3 floats), N - normal (3 floats), U - texture coordinates (2 floats), T - tangent (4 floats)",
  );
  console.log("Have fun!");
  process.exit(0);
}

if (!values.input || !fs.existsSync(values.input)) {
  console.error("No input file specified, or input file does not exist!");
  process.exit(1);
}

if (!values.output) {
  console.error("No output file specified!");
  process.exit(1);
}

if (!values.format) {
  console.error("No output format specified!");
  process.exit(1);
}

const format = values.format;
if (!format.match(/^[PNUT]{1,4}$/)) {
  console.error("Invalid format specified!");
  process.exit(1);
}
const formatChars = format.split("");
if (formatChars.filter((x) => x === "P").length > 1) {
  console.error("Invalid format specified!");
  process.exit(1);
}
if (formatChars.filter((x) => x === "N").length > 1) {
  console.error("Invalid format specified!");
  process.exit(1);
}
if (formatChars.filter((x) => x === "U").length > 1) {
  console.error("Invalid format specified!");
  process.exit(1);
}
if (formatChars.filter((x) => x === "T").length > 1) {
  console.error("Invalid format specified!");
  process.exit(1);
}

console.log(`Loading file ${values.input}`);

const objData = loadObjFileAsSingleGeometry(
  fs.readFileSync(values.input).toString("utf-8"),
);
const intermediate = objData.intermediate;
if (format.includes("T")) {
  intermediate.recalculateTangents();
}

console.log(`Generating raw data using format ${values.format}`);

const mappedComponents = formatChars.map((x) => {
  if (x === "P") return "position";
  if (x === "N") return "normal";
  if (x === "U") return "uv";
  if (x === "T") return "tangent";
  throw new Error(`Unknown component ${x}`);
});

const result = intermediate.getVertexArray(mappedComponents);

console.log(`Saving output to file ${values.output}`);
fs.writeFileSync(values.output, Buffer.from(result.data));
console.log(`Saving metadata output to file ${values.output}.json`);
fs.writeFileSync(
  values.output + ".json",
  JSON.stringify(result.layout, undefined, 2),
);

console.log(`Done!`);
