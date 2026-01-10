import { readComponentsMetadata } from "./readComponentsMetadata.js";
import { readApiExports } from "./readApiExports.js";
import * as fs from "node:fs";

const componentsMetadata = readComponentsMetadata();
const apiExports = readApiExports();

function camelize(str: string): string {
  return str[0].toLowerCase() + str.substring(1);
}

function camelizeSnake(str: string): string {
  return str
    .toLowerCase()
    .replace(/([-_][a-z])/g, (group) =>
      group.toUpperCase().replace("-", "").replace("_", ""),
    );
}

const apiExportImports = [
  ...new Set(
    [
      ...apiExports.commands.map((x) => x.importsTs),
      ...apiExports.events.map((x) => x.importsTs),
    ]
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
import { BaseGameApi } from "./BaseGameApi.js";

export class GameRawApi {
  private readonly api: BaseGameApi;
  
  public constructor(api: BaseGameApi){
    this.api = api;
  }`);

for (const command of apiExports.commands) {
  console.log(`
  public async ${camelizeSnake(command.name)}(${command.inputType.length === 0 ? "" : `input: ${command.inputType}`}): Promise<${command.returnType}> {
    return this.api.send({
      name: "command.${command.name}",
      payload: ${command.inputType.length === 0 ? "{}" : "input"},
    }) as Promise<${command.returnType}>;
  }
  `);
}

for (const component of componentsMetadata.filter((x) => x.type === "Option")) {
  console.log(`
  public const ${camelize(component.short)} = {
    get: (entityId: number): Promise<${component.full}> => {
      return this.api.send({
        name: "command.get_${component.snake}",
        payload: { entityId },
      }) as Promise<${component.full}>;
    },
    
    set: async (entityId: number, component: ${component.full}): Promise<void> => {
      await this.api.send({
        name: "command.set_${component.snake}",
        payload: { entityId, component },
      });
    },
    
    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_${component.snake}",
        payload: { entityId },
      });
    }
  }
`);
}

for (const component of componentsMetadata.filter((x) => x.type === "Vector")) {
  console.log(`
  public const ${camelize(component.short)} = {
    get: (entityId: number): Promise<${component.full}[]> => {
      return this.api.send({
        name: "command.get_${component.snake}",
        payload: { entityId },
      }) as Promise<${component.full}[]>;
    },
    
    add: async (entityId: number, component: ${component.full}): Promise<void> => {
      await this.api.send({
        name: "command.add_${component.snake}",
        payload: { entityId, component },
      });
    },
    
    remove: async (entityId: number, componentId: number): Promise<void> => {
      await this.api.send({
        name: "command.remove_${component.snake}",
        payload: { entityId, componentId },
      });
    }
  }
`);
}

for (const component of componentsMetadata.filter((x) => x.type === "Marker")) {
  console.log(`
  public const ${camelize(component.short)} = {
    get: (entityId: number): Promise<${component.full}> => {
      return this.api.send({
        name: "command.get_${component.snake}",
        payload: { entityId },
      }) as Promise<${component.full}>;
    },
    
    set: async (entityId: number, value: boolean): Promise<void> => {
      await this.api.send({
        name: "command.set_${component.snake}",
        payload: { entityId, value },
      });
    }
  }
`);
}

console.log("}"); // GameComponentsApi close