import { describe, it, afterAll, expect } from "vitest";
import { NATSTransport } from "../framework/natsTransport.js";
import { setTimeout } from "node:timers/promises";
import { startGame, startNATSServer } from "../framework/starter";
import { RemoteGameApi } from "../generated/RemoteGameApi";
import { BaseGameApi } from "../framework/BaseGameApi";
import { boot } from "./util/boot";

describe("test", () => {
  // afterAll(() => process.exit(0));

  it("has comms two sides", async () => {
    const gameApi = await boot(4321);

    console.time("timer");
    await gameApi.resetWorld();

    console.log(8);

    const serializedBefore = await gameApi.serializeWorld();
    // assert.strictEqual(serializedBefore.entities.length, 0);

    const { id: entityId } = await gameApi.addEntity({ components: null });
    await gameApi.isPlayer.set(entityId, true);

    const serializedAfter = await gameApi.serializeWorld();
    // assert.strictEqual(serializedAfter.entities.length, 1);
    // assert.strictEqual(await gameApi.isPlayer.get(entityId), true);

    console.timeEnd("timer");
  });

  it("Just adds entity successfully", async () => {
    const gameApi = await boot(4321);

    console.log(1);

    const { id: entityId } = await gameApi.addEntity({ components: null });

    console.log(2);

    console.log(entityId);
  });
});
