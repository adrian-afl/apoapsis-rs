import {
  emptyAttachedComponents,
  RemoteGameApi,
} from "../generated/RemoteGameApi";
import { BaseGameApi } from "../framework/BaseGameApi";
import { OnRawInputText, OnRawKeyDown } from "../generated/RemoteGameEvents";

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

  private promptString = "";
  private promptLabel: Label;
  private promptShown: boolean;

  public constructor(
    gameApi: RemoteGameApi,
    baseApi: BaseGameApi,
    onPrompt: (text: string) => void | Promise<void>,
  ) {
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

    this.promptLabel = new Label(gameApi);
    this.promptLabel.setPosition(0.01, 0.33);

    const tildeKey = 41;
    const backspaceKey = 14;
    const enterKey = 28;
    baseApi.subscribe(OnRawKeyDown, async (x) => {
      if (x.data === tildeKey) {
        this.promptShown = !this.promptShown;
        if (this.promptShown) {
          this.promptString = "";
          await this.promptLabel.setLabel("$: ");
        } else {
          await this.promptLabel.setLabel("");
        }
      } else if (this.promptShown) {
        if (x.data === backspaceKey) {
          this.promptString = this.promptString.substring(
            0,
            this.promptString.length - 1,
          );
          await this.promptLabel.setLabel(`$: ${this.promptString}`);
        } else if (x.data === enterKey) {
          onPrompt(this.promptString);
          await this.println(`>>> ${this.promptString}`);
          this.promptShown = false;
          await this.promptLabel.setLabel("");
        }
      }
    });

    baseApi.subscribe(OnRawInputText, async (x) => {
      console.log({
        value: x.data,
        test: x.data.match(/^[A-Za-z0-9 \\\/.,()[\]\-=+!@#$%^&*]+$/),
      });
      if (
        this.promptShown &&
        x.data.match(/^[A-Za-z0-9 \\\/.,()[\]\-=+!@#$%^&*]+$/) !== null
      ) {
        this.promptString += x.data;
        await this.promptLabel.setLabel(`$: ${this.promptString}`);
      }
    });
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
