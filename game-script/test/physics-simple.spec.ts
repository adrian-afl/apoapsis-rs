import { describe, it, afterAll, expect } from "vitest";
import { NATSTransport } from "../framework/natsTransport.js";
import { setTimeout } from "node:timers/promises";
import { startGame, startNATSServer } from "../framework/starter";
import { RemoteGameApi } from "../generated/RemoteGameApi";
import { BaseGameApi } from "../framework/BaseGameApi";
import { boot } from "./util/boot";
import type { UniverseClockComponent } from "../generated/types/UniverseClockComponent";
import type { FirstPersonCameraControlComponent } from "../generated/types/FirstPersonCameraControlComponent";
import type { ThirdPersonOrbitCameraControlComponent } from "../generated/types/ThirdPersonOrbitCameraControlComponent";
import type { ThirdPersonStaticCameraControlComponent } from "../generated/types/ThirdPersonStaticCameraControlComponent";
import type { TransformComponent } from "../generated/types/TransformComponent";
import type { RealPhysicsComponent } from "../generated/types/RealPhysicsComponent";
import type { SimplePhysicsComponent } from "../generated/types/SimplePhysicsComponent";
import type { SetPhysicsKinematicsComponent } from "../generated/types/SetPhysicsKinematicsComponent";
import type { MeshComponent } from "../generated/types/MeshComponent";
import type { ShipControlComponent } from "../generated/types/ShipControlComponent";
import type { UIColorComponent } from "../generated/types/UIColorComponent";
import type { UIHoverColorComponent } from "../generated/types/UIHoverColorComponent";
import type { UIBoxComponent } from "../generated/types/UIBoxComponent";
import type { UIHoverCursorComponent } from "../generated/types/UIHoverCursorComponent";
import type { UITextureComponent } from "../generated/types/UITextureComponent";
import type { UITextComponent } from "../generated/types/UITextComponent";

describe("physics simple tests", () => {
  // afterAll(() => process.exit(0));

  it("can spawn an entity with physics near a planet", async () => {
    const gameApi = await boot(4321);

    console.log(await gameApi.getAllCelestialBodyNames());

    const { id: entityId } = await gameApi.addEntity({
      components: {
        camera_focus: true,
        transform: {
          id: 123,
          orientation: [1.0, 0.0, 0.0, 1.0],
          position: { x: "1000.0", y: "200.0", z: "1.0" },
          scale: [1.0, 1.0, 1.0],
        },
        universe_clock: {
          id: 333,
          time: "1",
          should_advance: true,
        },
        first_person_camera_control: null,
        third_person_orbit_camera_control: null,
        third_person_static_camera_control: null,
        is_ground_collider: false,
        real_physics: null,
        simple_physics: null,
        set_physics_kinematics: [],
        is_player: false,
        mesh: [],
        control_focus: false,
        ship_control: null,
        ui_color: null,
        ui_hover_color: null,
        ui_box: null,
        ui_hover_cursor: null,
        ui_texture: null,
        ui_text: null,
        ui_is_raycastable: false,
        ui_require_free_cursor: false,
      },
    });

    await setTimeout(5000);

    const earthPosition = await gameApi.getCelestialBodyPosition("earth");
    console.log(earthPosition);
  });
});
