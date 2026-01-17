import { describe, it } from "vitest";
import { setTimeout } from "node:timers/promises";
import { emptyAttachedComponents } from "../generated/RemoteGameApi";
import { boot } from "./util/boot";
import { AttachedComponents } from "../generated/types/AttachedComponents";

describe("physics simple tests", () => {
  // afterAll(() => process.exit(0));

  it("can spawn an entity with physics near a planet", async () => {
    const gameApi = await boot(4321);

    console.log(await gameApi.getAllCelestialBodyNames());

    type AttachedComponentsWithoutIds = {
      [k in keyof AttachedComponents]: Omit<AttachedComponents[k], "id">;
    };

    function addComponents(
      base: AttachedComponents,
      news: Partial<AttachedComponentsWithoutIds>,
    ) {
      return { ...base, ...news };
    }

    type ComponentsBuilder = {
      build: () => AttachedComponents;
      add: (
        components: Partial<AttachedComponentsWithoutIds>,
      ) => ComponentsBuilder;
    };

    function fillInMissingIds(
      initial: Partial<AttachedComponentsWithoutIds>,
    ): Partial<AttachedComponents> {
      return Object.fromEntries(
        Object.entries(initial).map(([k, v]) => {
          let newv = v;
          if (typeof v === "object") {
            v["id"] = 0;
          }
          if (Array.isArray(v)) {
            newv = v.map((y) => {
              y["id"] = 0;
              return y;
            });
          }
          return [k, newv];
        }),
      ) as Partial<AttachedComponents>;
    }

    function createBuilder(data: AttachedComponents): ComponentsBuilder {
      return {
        build: () => data,
        add: (components: Partial<AttachedComponentsWithoutIds>) =>
          createBuilder({ ...data, ...fillInMissingIds(components) }),
      };
    }

    function buildComponents(): ComponentsBuilder {
      return createBuilder(emptyAttachedComponents);
    }

    console.log({
      components: buildComponents()
        .add({
          camera_focus: true,
          transform: {
            orientation: [1.0, 0.0, 0.0, 1.0],
            position: { x: "1000.0", y: "200.0", z: "1.0" },
            scale: [1.0, 1.0, 1.0],
          },
          universe_clock: {
            time: "1",
            should_advance: true,
          },
        })
        .build(),
    });
    const { id: entityId } = await gameApi.addEntity({
      components: buildComponents()
        .add({
          camera_focus: true,
          transform: {
            orientation: [1.0, 0.0, 0.0, 1.0],
            position: { x: "1000.0", y: "200.0", z: "1.0" },
            scale: [1.0, 1.0, 1.0],
          },
          universe_clock: {
            time: "1",
            should_advance: true,
          },
        })
        .build(),
    });

    await setTimeout(1000);

    const earthPosition = await gameApi.getCelestialBodyPosition("earth");
    console.log(earthPosition);
  });
});
