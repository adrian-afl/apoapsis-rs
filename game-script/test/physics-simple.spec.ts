import { describe, it } from "vitest";
import { setTimeout } from "node:timers/promises";
import { emptyAttachedComponents } from "../generated/RemoteGameApi";
import { boot } from "./util/boot";
import { AttachedComponents } from "../generated/types/AttachedComponents";
import DVec3 from "../framework/mathModule/logic/linear/DVec3";
import * as fs from "node:fs";
import { setInterval } from "node:timers";

describe("physics simple tests", () => {
  // afterAll(() => process.exit(0));

  it("can spawn an entity with physics near a planet", async () => {
    const { gameApi, kill } = await boot(7878, false);

    console.log(await gameApi.getAllCelestialBodyNames());

    function addComponents(
      base: AttachedComponents,
      news: Partial<AttachedComponents>,
    ) {
      return { ...base, ...news };
    }

    type ComponentsBuilder = {
      build: () => AttachedComponents;
      add: (components: Partial<AttachedComponents>) => ComponentsBuilder;
    };

    function createBuilder(data: AttachedComponents): ComponentsBuilder {
      return {
        build: () => data,
        add: (components: Partial<AttachedComponents>) =>
          createBuilder({ ...data, ...components }),
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
    const { id: playerEID } = await gameApi.addEntity({
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

    const radius = earthDef.terrain.radius + 40.9 * 1000.0;

    const newPosition = DVec3.fromDecimalVector3d(earthPosition).addVec3(
      DVec3.fromNumbers(0.0, 0.0, -radius),
    );

    await gameApi.replaceEntityComponents(
      playerEID,
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
          real_physics: {
            shape_description: {
              ball: {
                radius: 1.0,
              },
            },
            override_real_simulation_cutoff: null,
          },
          // glue_to_celestial_body: {
          //   bodyName: "earth",
          //   offset: [0.0, 0.0, -radius],
          //   orientation: [1.0, 0.0, 0.0, 1.0],
          // },
        })
        .build(),
    );

    const { id: testBoxEID } = await gameApi.addEntity({
      components: buildComponents()
        .add({
          transform: {
            orientation: [1.0, 0.0, 0.0, 1.0],
            position: newPosition
              .addVec3(DVec3.fromNumbers(-100.0, 0.0, 0.0))
              .toDecimalVector3d(),
            scale: [1.0, 1.0, 1.0],
          },
          simple_physics: {
            angular_velocity: [0.0, 0.0, 0.0],
            mass: "1.0",
            linear_velocity: [0.0, 0.0, 0.0],
          },
          real_physics: {
            shape_description: {
              ball: {
                radius: 1.0,
              },
            },
            override_real_simulation_cutoff: null,
          },
          mesh: [
            {
              description: {
                geometryPath: "media/smoothbox.pnut.raw",
                material: {
                  color: {
                    color: [1.0, 1.0, 1.0],
                  },
                  roughness: {
                    value: 1.0,
                  },
                  metalness: {
                    value: 0.0,
                  },
                  emission: {
                    color: [1.0, 0.0, 0.0],
                  },
                  bump: null,
                  normal: null,
                },
              },
            },
          ],
        })
        .build(),
    });

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

    // console.log(
    //   DVec3.fromDecimalVector3d(
    //     (await gameApi.transform.get(entityId)).position,
    //   ).distanceTo(DVec3.fromDecimalVector3d(earthPosition)),
    // );
    setInterval(async () => {
      const transform = await gameApi.transform.get(testBoxEID);
      const simple_physics = await gameApi.simplePhysics.get(testBoxEID);
      const altitude = await gameApi.getRealAltitudeOverCelestialBody(
        "earth",
        transform.position,
      );

      console.log(altitude, simple_physics.linear_velocity);

      await gameApi.uIText.set(labelId, {
        color: [1.0, 1.0, 1.0, 1.0],
        content: altitude.terrain.toString() + " - " + new Date().toISOString(),
        font_size: "Medium",
      });
    }, 500.0);
    await setTimeout(10000.0);

    fs.writeFileSync(
      "debug-ecs.json",
      JSON.stringify(await gameApi.serializeWorld(), undefined, 2),
    );
    fs.writeFileSync(
      "points.json",
      JSON.stringify(
        await gameApi.getDebugRealPhysicsWireframe(),
        undefined,
        2,
      ),
    );

    await setTimeout(1000000.0);

    // console.log(await gameApi.getDebugRealPhysicsWireframe());
    //
    // fs.writeFileSync(
    //   "debug-ecs.json",
    //   JSON.stringify(await gameApi.serializeWorld(), undefined, 2),
    // );
    // fs.writeFileSync(
    //   "points.json",
    //   JSON.stringify(
    //     await gameApi.getDebugRealPhysicsWireframe(),
    //     undefined,
    //     2,
    //   ),
    // );

    // kill();

    // await setTimeout(10 * 1000.0);
  });
});
