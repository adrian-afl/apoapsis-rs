import { startGame } from "../../framework/starter";
import { TCPTransport } from "../../framework/transport";
import { BaseGameApi } from "../../framework/BaseGameApi";
import { RemoteGameApi } from "../../generated/RemoteGameApi";
import { setTimeout } from "node:timers/promises";
import { waitForEvent } from "./waitForEvent";
import { OnGameBootReady } from "../../generated/RemoteGameEvents";

export async function boot(port: number, headless?: boolean) {
  const client = new TCPTransport(`localhost:${port}`);
  const baseApi = new BaseGameApi((message) => client.send(message));
  const gameApi = new RemoteGameApi(baseApi);
  client.setOnReceive((message) => baseApi.receive(message));
  const gameInstance = await startGame(
    "release",
    port,
    headless !== undefined ? headless : true,
  );
  // await setTimeout(1000);
  await client.connect();

  console.log("Waiting for OnGameBootReady event");
  await waitForEvent(baseApi, OnGameBootReady);
  console.log("DONE Waiting for OnGameBootReady event");

  return {
    gameApi,
    kill: () => {
      gameInstance.kill();
    },
  };
}
