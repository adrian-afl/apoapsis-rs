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

describe("latency testing", () => {
  // afterAll(() => process.exit(0));

  it("measures latency", async () => {
    const gameApi = await boot(4321);

    console.log("Booted");

    let last = Date.now();
    let avgDiff = 0.0;
    let avgDiffWeight = 0.0;
    while (true) {
      console.log(await gameApi.getAllCelestialBodyNames());
      const now = Date.now();
      const diff = now - last;
      last = now;
      avgDiff += diff;
      avgDiffWeight += 1.0;

      console.log(
        `Avg time: ${avgDiffWeight > 0 ? avgDiff / avgDiffWeight : "NaN"}`,
      );
    }
  });
});
