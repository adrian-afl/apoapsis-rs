import {
  emptyAttachedComponents,
  RemoteGameApi,
} from "../generated/RemoteGameApi";
import { BaseGameApi } from "../framework/BaseGameApi";
import { OnRawKeyDown } from "../generated/RemoteGameEvents";

export class Label {
  public gameApi: RemoteGameApi;
  private entityId: number = -1;

  public constructor(gameApi: RemoteGameApi) {
    this.gameApi = gameApi;
  }

  private async initialize() {
    this.entityId = (
      await this.gameApi.addEntity({
        components: {
          ...emptyAttachedComponents,
          ui_text: {
            color: [1.0, 1.0, 1.0, 1.0],
            content: "",
            font_size: "Small",
          },
          ui_box: {
            orientation: 0,
            z_index: 1,
            position: [0.5, 0.5],
            size: [0.1, 0.1],
          },
          ui_color: {
            color: [0.0, 0.0, 0.0, 0.0],
          },
        },
      })
    ).id;
  }

  public async destroy() {
    await this.gameApi.removeEntity(this.entityId);
    this.entityId = -1;
  }

  public async setPosition(x: number, y: number) {
    if (this.entityId === -1) await this.initialize();
    await this.gameApi.uIBox.set(this.entityId, {
      orientation: 0,
      z_index: 1,
      position: [x, y],
      size: [1.0, 1.0],
    });
  }

  public async setLabel(label: string) {
    if (this.entityId === -1) await this.initialize();
    await this.gameApi.uIText.set(this.entityId, {
      color: [1.0, 1.0, 1.0, 1.0],
      content: label,
      font_size: "Small",
    });
  }
}
