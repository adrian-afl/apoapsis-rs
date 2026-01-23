import { describe, it } from "vitest";
import { boot } from "./util/boot";

describe("latency testing", () => {
  // afterAll(() => process.exit(0));

  it("measures latency", async () => {
    const { gameApi, kill } = await boot(7878, true);

    console.log("Booted");

    let last = Date.now();
    let avgDiff = 0.0;
    let avgDiffWeight = 0.0;
    let i = 0;
    while (true) {
      await gameApi.getAllCelestialBodyNames();
      const now = Date.now();
      const diff = now - last;
      last = now;
      avgDiff += diff;
      avgDiffWeight += 1.0;

      if (i++ % 100 === 0) {
        console.log(
          `Avg time: ${avgDiffWeight > 0 ? avgDiff / avgDiffWeight : "NaN"}`,
        );
      }
    }
  });
});
