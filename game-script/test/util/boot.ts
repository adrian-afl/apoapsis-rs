import { startGame, startNATSServer } from "../../framework/starter";
import { NATSTransport } from "../../framework/natsTransport";
import { BaseGameApi } from "../../framework/BaseGameApi";
import { RemoteGameApi } from "../../generated/RemoteGameApi";
import { setTimeout } from "node:timers/promises";
import { waitForEvent } from "./waitForEvent";
import { OnGameBootReady } from "../../generated/RemoteGameEvents";

export async function boot(port: number) {
  const natsServer = startNATSServer(port);
  const nats = new NATSTransport(`localhost:${port}`);
  const baseApi = new BaseGameApi((message) => nats.send(message));
  const gameApi = new RemoteGameApi(baseApi);
  nats.setOnReceive((message) => baseApi.receive(message));
  const gameInstance = startGame("release", port, true);
  while (true) {
    try {
      await nats.connect();
      break;
    } catch {
      await setTimeout(100);
    }
  }

  await waitForEvent(baseApi, OnGameBootReady);

  return gameApi;
}
