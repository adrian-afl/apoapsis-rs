import { describe, expect, it } from "vitest";
import { boot } from "./util/boot";

describe("test", () => {
  // afterAll(() => process.exit(0));

  it("has comms two sides", async () => {
    const { gameApi, kill } = await boot(4343, true);

    console.time("timer");
    await gameApi.resetWorld();

    const serializedBefore = await gameApi.serializeWorld();
    // assert.strictEqual(serializedBefore.entities.length, 0);

    const { id: entityId } = await gameApi.addEntity({ components: null });
    await gameApi.isPlayer.set(entityId, true);

    const serializedAfter = await gameApi.serializeWorld();
    console.timeEnd("timer");

    expect(serializedAfter.entities.length).toStrictEqual(1);
    expect(await gameApi.isPlayer.get(entityId)).toStrictEqual(true);
  });

  it("Just adds entity successfully", async () => {
    const gameApi = await boot(4321);

    console.log(1);

    const { id: entityId } = await gameApi.addEntity({ components: null });

    console.log(2);

    console.log(entityId);
  });
});
