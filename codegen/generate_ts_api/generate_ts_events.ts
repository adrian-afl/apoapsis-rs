import { readComponentsMetadata } from "./readComponentsMetadata.js";
import { readApiExports } from "./readApiExports.js";

const componentsMetadata = readComponentsMetadata();
const apiExports = readApiExports();

function pascalize(str: string): string {
  return str[0].toUpperCase() + str.substring(1);
}

function camelizeSnake(str: string): string {
  return str
    .toLowerCase()
    .replace(/([-_][a-z])/g, (group) =>
      group.toUpperCase().replace("-", "").replace("_", ""),
    );
}

function pascalizeSnake(str: string): string {
  return pascalize(camelizeSnake(str));
}

const apiExportImports = [
  ...new Set(
    [...apiExports.events.map((x) => x.importsTs)]
      .flat()
      .map((x) => x.trim().replaceAll("[]", ""))
      .filter(
        (x) =>
          !x.endsWith('void.js";') &&
          !x.endsWith('null.js";') &&
          !x.endsWith('number.js";') &&
          !x.endsWith('string.js";'),
      ),
  ),
];

console.log(`${componentsMetadata.map((x) => x.importTs).join("\n")}
${apiExportImports.join("\n")}

export abstract class AbstractBaseEvent {
  public eventName = this.constructor.name;
}
`);
for (const event of apiExports.events) {
  console.log(`
export class ${pascalizeSnake(event.name)} extends AbstractBaseEvent {
      ${
        event.payloadType
          ? `
      public readonly data: ${event.payloadType};
      public constructor(input: ${event.payloadType}){
        super();
        this.data = input;
      }
      `
          : ""
      }
}
`);
}

console.log("export const eventsConstructors = {");

for (const event of apiExports.events) {
  console.log(
    `${event.name}: (${event.payloadType ? `input: ${event.payloadType}` : ""}) => new ${pascalizeSnake(event.name)}(${event.payloadType ? `input` : ""}),`,
  );
}

console.log("}; // eventsConstructors close");
