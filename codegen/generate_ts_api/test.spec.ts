import { describe, it, after } from "node:test";
import * as assert from "node:assert";
import { NATSTransport } from "./natsTransport.js";
import { GameApi } from "./GameApi.js";
import { GameCommandsApi, GameComponentsApi } from "./generated_ts_client.js";
import { setTimeout } from "node:timers/promises";
import { startGame, startNATSServer } from "./starter.js";

describe("test", () => {
  after(() => process.exit(0));

  it("has comms two sides", async () => {
    const port = 4321;

    const natsServer = startNATSServer(port);

    console.log(1);

    const nats = new NATSTransport(`localhost:${port}`);
    const gameApi = new GameApi((message) => nats.send(message));

    console.log(2);

    const componentsApi = new GameComponentsApi(gameApi);
    const commandsApi = new GameCommandsApi(gameApi);

    console.log(3);

    nats.setOnReceive((message) => gameApi.receive(message));

    await setTimeout(1000);
    await nats.connect();

    console.log(4);

    const gameInstance = startGame("release", port, true);

    await setTimeout(5000);

    console.time("timer");
    await commandsApi.resetWorld();

    console.log(5);

    const serializedBefore = await commandsApi.serializeWorld();
    assert.strictEqual(serializedBefore.entities.length, 0);

    const { id: entityId } = await commandsApi.createEntity({ name: "test" });
    await componentsApi.isPlayer.set(entityId, true);

    const serializedAfter = await commandsApi.serializeWorld();
    assert.strictEqual(serializedAfter.entities.length, 1);
    assert.strictEqual(await componentsApi.isPlayer.get(entityId), true);

    console.timeEnd("timer");
  });
});
