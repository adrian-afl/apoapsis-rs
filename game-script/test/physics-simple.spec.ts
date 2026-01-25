import { describe, it } from "vitest";
import { setTimeout } from "node:timers/promises";
import { emptyAttachedComponents } from "../generated/RemoteGameApi";
import { boot } from "./util/boot";
import { AttachedComponents } from "../generated/types/AttachedComponents";
import DVec3 from "../framework/mathModule/logic/linear/DVec3";
import * as fs from "node:fs";
import { setInterval } from "node:timers";
import Decimal from "decimal.js";
import { Quaternion } from "@aeroflightlabs/linear-math";
import { DecimalVector3d } from "../generated/types/DecimalVector3d";
import { OnPhysicsCollisionEvent } from "../generated/RemoteGameEvents";
import { DebugDisplay } from "../script/debugDisplay";
import { dec } from "../framework/mathModule/decimalHelpers";

describe("physics simple tests", () => {
  // afterAll(() => process.exit(0));

  it("can spawn an entity with physics near a planet", async () => {
    const { gameApi, baseApi, kill } = await boot(7878, false);

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

    const radius = earthDef.terrain.radius + 37.51 * 1000.0;

    const newPosition = DVec3.fromDecimalVector3d(earthPosition).addVec3(
      DVec3.fromNumbers(100000.0, 0.0, -radius),
    );

    const quatStr = (x: Quaternion) =>
      [x.x, x.y, x.z, x.w] as [number, number, number, number];

    const decvec3Arr = (x: DecimalVector3d) =>
      [x.x, x.y, x.z] as [number, number, number];

    await gameApi.replaceEntityComponents(
      playerEID,
      buildComponents()
        .add({
          camera_focus: true,
          transform: {
            orientation: quatStr(new Quaternion().identity()),
            position: newPosition.toDecimalVector3d(),
            scale: [1.0, 1.0, 1.0],
          },
          // first_person_camera_control: {
          // fov: 70.0,
          // },
          third_person_orbit_camera_control: {
            style: "Absolute",
            initial_offset: [0.0, 0.0, -5.0],
            initial_orientation: [1.0, 0.0, 0.0, 1.0],
            fov: 70.0,
          },
          is_player: true,
          control_focus: true,
          ui_require_free_cursor: false,
          universe_clock: {
            time: "1",
            should_advance: false,
          },
          simple_physics: {
            angular_velocity: [1.0, 1.0, 1.0],
            mass: "1.0",
            linear_velocity: [0.0, 0.0, 0.0],
            // linear_velocity: DVec3.fromDecimalVector3d(
            //   await gameApi.getCelestialBodySurfaceVelocity(
            //     "earth",
            //     DVec3.fromNumbers(100000.0, 0.0, -radius).toDecimalVector3d(),
            //   ),
            // ).asNumbers(),
          },
          real_physics: {
            shape_description: {
              box: {
                sizeX: 1.0,
                sizeY: 1.0,
                sizeZ: 1.0,
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
            {
              description: {
                geometryPath: "media/axes.pnut.raw",
                material: {
                  color: {
                    color: [1.0, 0.0, 1.0],
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
          // glue_to_celestial_body: {
          //   bodyName: "earth",
          //   offset: [0.0, 0.0, -radius],
          //   orientation: [1.0, 0.0, 0.0, 1.0],
          // },
        })
        .build(),
    );
    //
    // const { id: testBoxEID } = await gameApi.addEntity({
    //   components: buildComponents()
    //     .add({
    //       transform: {
    //         orientation: quatStr(new Quaternion().identity()),
    //         position: newPosition
    //           .addVec3(DVec3.fromNumbers(10.0, 0.0, 10.0))
    //           .toDecimalVector3d(),
    //         scale: [1.0, 1.0, 1.0],
    //       },
    //       simple_physics: {
    //         angular_velocity: [0.0, 0.0, 0.0],
    //         mass: "1.0",
    //         linear_velocity: [0.0, 0.0, 0.0],
    //       },
    //       real_physics: {
    //         shape_description: {
    //           ball: {
    //             radius: 1.0,
    //           },
    //         },
    //         override_real_simulation_cutoff: null,
    //       },
    //       mesh: [
    //         {
    //           description: {
    //             geometryPath: "media/smoothbox.pnut.raw",
    //             material: {
    //               color: {
    //                 color: [1.0, 1.0, 1.0],
    //               },
    //               roughness: {
    //                 value: 1.0,
    //               },
    //               metalness: {
    //                 value: 0.0,
    //               },
    //               emission: {
    //                 color: [1.0, 0.0, 0.0],
    //               },
    //               bump: null,
    //               normal: null,
    //             },
    //           },
    //         },
    //       ],
    //     })
    //     .build(),
    // });

    // console.log(
    //   DVec3.fromDecimalVector3d(
    //     (await gameApi.transform.get(entityId)).position,
    //   ).distanceTo(DVec3.fromDecimalVector3d(earthPosition)),
    // );

    baseApi.subscribe(OnPhysicsCollisionEvent, (e) => {
      console.log(e);
    });

    const debugDisplay = new DebugDisplay(gameApi);

    await debugDisplay.println("Hello world");
    await debugDisplay.println("Another line");
    await debugDisplay.println("Test!!!!!!!");

    await debugDisplay.println("Hello world");
    await debugDisplay.println("Another line");
    await debugDisplay.println("Test!!!!!!!");

    await debugDisplay.println("Hello world");
    await debugDisplay.println("Another line");
    await debugDisplay.println("Test!!!!!!!");

    await debugDisplay.println("Hello world");
    await debugDisplay.println("Another line");
    await debugDisplay.println("Test!!!!!!!");

    await debugDisplay.debug("Altitude", "123.0");
    await debugDisplay.debug("Raycast", "123.0");
    await debugDisplay.debug("LinVel", "123.0");
    await debugDisplay.debug("Now", "123.0");

    const main = async () => {
      const [transform, simple_physics] = await Promise.all([
        gameApi.transform.get(playerEID),
        gameApi.simplePhysics.get(playerEID),
      ]);
      const [altitude, raycast] = await Promise.all([
        gameApi.getApproximateAltitudeOverCelestialBody(
          "earth",
          transform.position,
        ),
        gameApi.raycastRealPhysics(
          DVec3.fromNumbersArray(simple_physics.linear_velocity)
            .normalized()
            .mulScalar(new Decimal(3.0))
            .asNumbers(),
          DVec3.fromDecimalVector3d(earthPosition)
            .subVec3(newPosition)
            .normalized()
            .asNumbers(),
        ),
      ]);

      // console.log(
      //   altitude,
      //   transform.position,
      //   simple_physics.linear_velocity,
      //   raycast,
      // );
      await Promise.all([
        debugDisplay.debug("Altitude", dec(altitude).toFixed(4)),
        debugDisplay.debug("Raycast", raycast ? raycast.toFixed(4) : "null"),
        debugDisplay.debug(
          "LinVel",
          DVec3.fromNumbersArray(simple_physics.linear_velocity).toString(4),
        ),
        debugDisplay.debug("Now", new Date().toLocaleString()),
      ]);

      // await gameApi.uIText.set(labelId, {
      //   color: [1.0, 1.0, 1.0, 1.0],
      //   content: altitude.toString(),
      //   font_size: "Medium",
      // });
      //
      // console.log("main fin");

      global.setTimeout(async () => {
        await main();
      }, 1);
    };

    await setTimeout(1000.0);
    void main();

    await setTimeout(1000000.0);

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
    //
    // await setTimeout(1000000.0);

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
