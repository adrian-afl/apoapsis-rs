import { readComponentsMetadata } from "./readComponentsMetadata.js";

const componentsMetadata = readComponentsMetadata();

console.log(`${componentsMetadata.map((x) => x.importTs).join("\n")}
import { GameApi } from "./GameApi.js";

export class GameComponentsApi {
  private readonly api: GameApi;
  
  public constructor(api: GameApi){
    this.api = api;
  }`);

for (const component of componentsMetadata.filter((x) => x.type === "Option")) {
  console.log(`
  public get${component.short}(entityId: number): Promise<${component.full}> {
    return this.api.send({
      name: "command.get_${component.snake}",
      payload: { entityId },
    }) as Promise<${component.full}>;
  }
  
  public async set${component.short}(entityId: number, component: ${component.full}): Promise<void> {
    await this.api.send({
      name: "command.set_${component.snake}",
      payload: { entityId, component },
    });
  }
  
  public async clear${component.short}(entityId: number): Promise<void> {
    await this.api.send({
      name: "command.clear_${component.snake}",
      payload: { entityId },
    });
  }
`);
}

for (const component of componentsMetadata.filter((x) => x.type === "Vector")) {
  console.log(`
  public get${component.short}(entityId: number): Promise<${component.full}[]> {
    return this.api.send({
      name: "command.get_${component.snake}",
      payload: { entityId },
    }) as Promise<${component.full}[]>;
  }
  
  public async add${component.short}(entityId: number, component: ${component.full}): Promise<void> {
    await this.api.send({
      name: "command.add_${component.snake}",
      payload: { entityId, component },
    });
  }
  
  public async remove${component.short}(entityId: number, componentId: number): Promise<void> {
    await this.api.send({
      name: "command.remove_${component.snake}",
      payload: { entityId, componentId },
    });
  }
`);
}

for (const component of componentsMetadata.filter((x) => x.type === "Marker")) {
  console.log(`
  public get${component.short}(entityId: number): Promise<${component.full}> {
    return this.api.send({
      name: "command.get_${component.snake}",
      payload: { entityId },
    }) as Promise<${component.full}>;
  }
  
  public async set${component.short}(entityId: number, value: boolean): Promise<void> {
    await this.api.send({
      name: "command.set_${component.snake}",
      payload: { entityId, value },
    });
  }

`);
}

console.log("}"); // GameComponentsApi close
