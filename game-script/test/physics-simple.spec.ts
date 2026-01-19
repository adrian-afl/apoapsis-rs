import { describe, it } from "vitest";
import { setTimeout } from "node:timers/promises";
import { emptyAttachedComponents } from "../generated/RemoteGameApi";
import { boot } from "./util/boot";
import { AttachedComponents } from "../generated/types/AttachedComponents";
import DVec3 from "../framework/mathModule/logic/linear/DVec3";

describe("physics simple tests", () => {
  // afterAll(() => process.exit(0));

  it("can spawn an entity with physics near a planet", async () => {
    const gameApi = await boot(4321, false);

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
            should_advance: false,
          },
        })
        .build(),
    });

    await setTimeout(1000);

    const earthPosition = await gameApi.getCelestialBodyPosition("earth");
    console.log(earthPosition);

    const earthDef = await gameApi.getCelestialBodyDefinition("earth");
    console.log(earthDef);

    const radius = earthDef.terrain.radius + 10.0 * 1000.0; // 400 km over surface

    const newPosition = DVec3.fromDecimalVector3d(earthPosition).addVec3(
      DVec3.fromNumbers(0.0, 0.0, -radius),
    );

    await gameApi.replaceEntityComponents(
      entityId,
      buildComponents()
        .add({
          camera_focus: true,
          transform: {
            orientation: [1.0, 0.0, 0.0, 1.0],
            position: newPosition.toDecimalVector3d(),
            scale: [1.0, 1.0, 1.0],
          },
          first_person_camera_control: {
            fov: 70.0,
          },
          is_player: true,
          control_focus: true,
          ui_require_free_cursor: true,
          universe_clock: {
            time: "1",
            should_advance: false,
          },
          simple_physics: {
            angular_velocity: [0.0, 0.0, 0.0],
            mass: "1.0",
            linear_velocity: [0.0, 0.0, 0.0],
          },
          glue_to_celestial_body: {
            bodyName: "earth",
            offset: [0.0, 0.0, -radius],
            orientation: [1.0, 0.0, 0.0, 1.0],
          },
        })
        .build(),
    );

    const { id: labelId } = await gameApi.addEntity({
      components: buildComponents()
        .add({
          ui_text: {
            color: [1.0, 1.0, 1.0, 1.0],
            content: "Keke",
            font_size: "Medium",
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
        })
        .build(),
    });

    console.log(
      DVec3.fromDecimalVector3d(
        (await gameApi.transform.get(entityId)).position,
      ).distanceTo(DVec3.fromDecimalVector3d(earthPosition)),
    );

    while (true) {
      const altitude = DVec3.fromDecimalVector3d(
        (await gameApi.transform.get(entityId)).position,
      )
        .distanceTo(DVec3.fromDecimalVector3d(earthPosition))
        .sub(earthDef.terrain.radius);

      await gameApi.uIText.set(labelId, {
        id: 0,
        color: [1.0, 1.0, 1.0, 1.0],
        content: altitude.toString(),
        font_size: "Medium",
      });
      await setTimeout(100.0);
    }

    // await setTimeout(10 * 1000.0);
  });
});
