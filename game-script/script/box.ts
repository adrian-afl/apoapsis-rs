import {
  emptyAttachedComponents,
  RemoteGameApi,
} from "../generated/RemoteGameApi";
import { BaseGameApi } from "../framework/BaseGameApi";
import { OnRawKeyDown } from "../generated/RemoteGameEvents";
import { UICursorType } from "../generated/types/UICursorType";

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
        },
      })
    ).id;
  }

  public async destroy() {
    await this.gameApi.removeEntity(this.entityId);
    this.entityId = -1;
  }

  public getEntityId() {
    return this.entityId;
  }

  public async setPosition(x: number, y: number) {
    if (this.entityId === -1) await this.initialize();
    const old = await this.gameApi.uIBox.get(this.entityId);
    await this.gameApi.uIBox.set(this.entityId, {
      ...old,
      position: [x, y],
    });
  }

  public async setSize(width: number, height: number) {
    if (this.entityId === -1) await this.initialize();
    const old = await this.gameApi.uIBox.get(this.entityId);
    await this.gameApi.uIBox.set(this.entityId, {
      ...old,
      size: [width, height],
    });
  }

  public async setZIndex(zIndex: number) {
    if (this.entityId === -1) await this.initialize();
    const old = await this.gameApi.uIBox.get(this.entityId);
    await this.gameApi.uIBox.set(this.entityId, {
      ...old,
      z_index: zIndex,
    });
  }

  public async setColor(color: [number, number, number, number]) {
    if (this.entityId === -1) await this.initialize();
    await this.gameApi.uIColor.set(this.entityId, {
      color,
    });
  }

  public async setHoverColor(color: [number, number, number, number]) {
    if (this.entityId === -1) await this.initialize();
    await this.gameApi.uIHoverColor.set(this.entityId, {
      color,
    });
  }

  public async setHoverCursor(type: UICursorType) {
    if (this.entityId === -1) await this.initialize();
    await this.gameApi.uIHoverCursor.set(this.entityId, {
      typ: type,
    });
  }
}
