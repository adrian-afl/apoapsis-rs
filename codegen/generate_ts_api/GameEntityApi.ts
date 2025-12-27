import { GameApi } from "./GameApi.js";

export class GameEntityApi {
  private readonly api: GameApi;

  public constructor(api: GameApi) {
    this.api = api;
  }

  public createEntity(name?: string): Promise<{ id: number }> {
    return this.api.send({
      name: "command.create_entity",
      payload: { name: name ?? null },
    }) as Promise<{ id: number }>;
  }
}
