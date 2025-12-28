import { describe, it } from "node:test";
import * as assert from "node:assert";
import { NATSTransport } from "./natsTransport.js";
import { GameApi } from "./GameApi.js";
import { GameComponentsApi } from "./generated_ts_client.js";
import { GameECSWorldApi } from "./GameECSWorldApi.js";
import { GameEntityApi } from "./GameEntityApi.js";
import { setTimeout } from "node:timers/promises";

describe("test", () => {
  it("has comms two sides", async () => {
    const nats = new NATSTransport("localhost");
    const gameApi = new GameApi((message) => nats.send(message));

    const componentsApi = new GameComponentsApi(gameApi);
    const ecsWorldApi = new GameECSWorldApi(gameApi);
    const entityApi = new GameEntityApi(gameApi);

    nats.setOnReceive((message) => gameApi.receive(message));

    await nats.connect();

    await setTimeout(1000);

    console.time("timer");
    await ecsWorldApi.resetWorld();

    const serializedBefore = await ecsWorldApi.serializeWorld();
    assert.strictEqual(serializedBefore.entities.length, 0);

    const { id: entityId } = await entityApi.createEntity("test");
    await componentsApi.isPlayer.set(entityId, true);

    const serializedAfter = await ecsWorldApi.serializeWorld();
    assert.strictEqual(serializedAfter.entities.length, 1);
    assert.strictEqual(await componentsApi.isPlayer.get(entityId), true);

    console.timeEnd("timer");
  });
});
