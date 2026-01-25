import {
  emptyAttachedComponents,
  RemoteGameApi,
} from "../generated/RemoteGameApi";
import { BaseGameApi } from "../framework/BaseGameApi";

export class Modal {
  public gameApi: RemoteGameApi;
  public constructor(
    gameApi: RemoteGameApi,
    baseApi: BaseGameApi,
    content: string,
    buttons: { label: string; value: string }[],
  ) {
    this.gameApi = gameApi;
  }

  private async createBox(input: {
    x: number;
    y: number;
    width: number;
    height: number;
    zIndex: number;
    color: [number, number, number, number];
  }): Promise<number> {
    return (
      await this.gameApi.addEntity({
        components: {
          ...emptyAttachedComponents,
          ui_box: {
            orientation: 0,
            z_index: 1,
            position: [input.x, input.y],
            size: [input.width, input.height],
          },
          ui_color: {
            color: input.color,
          },
        },
      })
    ).id;
  }
}
