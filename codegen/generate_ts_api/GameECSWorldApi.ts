import { GameApi } from "./GameApi.js";
import { ECSWorldSerializedRepresentation } from "./types/ECSWorldSerializedRepresentation.js";

export class GameECSWorldApi {
  private readonly api: GameApi;

  public constructor(api: GameApi) {
    this.api = api;
  }

  public async resetWorld(): Promise<void> {
    await this.api.send({
      name: "command.reset_world",
      payload: {},
    });
  }

  public async serializeWorld(): Promise<ECSWorldSerializedRepresentation> {
    return this.api.send({
      name: "command.serialize_world",
      payload: {},
    }) as Promise<ECSWorldSerializedRepresentation>;
  }

  public async deserializeWorld(
    world: ECSWorldSerializedRepresentation,
  ): Promise<void> {
    (await this.api.send({
      name: "command.deserialize_world",
      payload: world,
    })) as Promise<ECSWorldSerializedRepresentation>;
  }
}
