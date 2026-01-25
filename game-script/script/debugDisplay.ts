import {
  emptyAttachedComponents,
  RemoteGameApi,
} from "../generated/RemoteGameApi";

class Label {
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

export class DebugDisplay {
  public gameApi: RemoteGameApi;
  private debugValues: Record<string, Label>;
  private consoleLabels: Label[];
  private consoleStrings: string[];

  public constructor(gameApi: RemoteGameApi) {
    this.gameApi = gameApi;
    this.debugValues = {};
    this.consoleLabels = [];
    this.consoleStrings = [];
    for (let i = 0; i < 10; i++) {
      this.consoleLabels.push(new Label(gameApi));
      this.consoleStrings.push("");
    }
    void (async () => {
      for (let i = 0; i < 10; i++) {
        await this.consoleLabels[i].setPosition(0.01, 0.01 + 0.03 * i);
      }
    })();
  }

  public async println(line: string) {
    this.consoleStrings.shift();
    this.consoleStrings.push(line);

    await Promise.all(
      this.consoleLabels.map(async (v, i) =>
        v.setLabel(this.consoleStrings[i]),
      ),
    );
  }

  public async debug(key: string, value: string) {
    if (!this.debugValues[key]) {
      this.debugValues[key] = new Label(this.gameApi);
      const allKeys = Object.keys(this.debugValues);
      await Promise.all([
        this.debugValues[key].setLabel(value),
        ...allKeys.map(async (k, i) =>
          this.debugValues[k].setPosition(0.01, 0.97 - 0.03 * i),
        ),
      ]);
    }
    await this.debugValues[key].setLabel(`[${key}]: ${value}`);
  }
}
