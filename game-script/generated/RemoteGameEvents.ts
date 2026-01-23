import { UniverseClockComponent } from "./types/UniverseClockComponent.js";
import { CameraFocusComponent } from "./types/CameraFocusComponent.js";
import { FirstPersonCameraControlComponent } from "./types/FirstPersonCameraControlComponent.js";
import { ThirdPersonOrbitCameraControlComponent } from "./types/ThirdPersonOrbitCameraControlComponent.js";
import { ThirdPersonStaticCameraControlComponent } from "./types/ThirdPersonStaticCameraControlComponent.js";
import { TransformComponent } from "./types/TransformComponent.js";
import { IsGroundColliderComponent } from "./types/IsGroundColliderComponent.js";
import { RealPhysicsComponent } from "./types/RealPhysicsComponent.js";
import { SimplePhysicsComponent } from "./types/SimplePhysicsComponent.js";
import { SetPhysicsKinematicsComponent } from "./types/SetPhysicsKinematicsComponent.js";
import { GlueToCelestialBodyComponent } from "./types/GlueToCelestialBodyComponent.js";
import { IsCelestialBodySurfaceComponent } from "./types/IsCelestialBodySurfaceComponent.js";
import { IsPlayerComponent } from "./types/IsPlayerComponent.js";
import { MeshComponent } from "./types/MeshComponent.js";
import { ControlFocusComponent } from "./types/ControlFocusComponent.js";
import { ShipControlComponent } from "./types/ShipControlComponent.js";
import { UIColorComponent } from "./types/UIColorComponent.js";
import { UIHoverColorComponent } from "./types/UIHoverColorComponent.js";
import { UIBoxComponent } from "./types/UIBoxComponent.js";
import { UIHoverCursorComponent } from "./types/UIHoverCursorComponent.js";
import { UITextureComponent } from "./types/UITextureComponent.js";
import { UITextComponent } from "./types/UITextComponent.js";
import { UIIsRaycastableComponent } from "./types/UIIsRaycastableComponent.js";
import { UIRequireFreeCursorComponent } from "./types/UIRequireFreeCursorComponent.js";
import { OnGameBootReadyEventData } from "./types/OnGameBootReadyEventData.js";

export abstract class AbstractBaseEvent {
  public eventName = this.constructor.name;
}

export class OnRemoteGameModeInitialized extends AbstractBaseEvent {}

export class OnNatsConnected extends AbstractBaseEvent {}

export class OnGameBootReady extends AbstractBaseEvent {
  public readonly data: OnGameBootReadyEventData;
  public constructor(input: OnGameBootReadyEventData) {
    super();
    this.data = input;
  }
}

export class Startup extends AbstractBaseEvent {}

export const eventsConstructors = {
  on_remote_game_mode_initialized: () => new OnRemoteGameModeInitialized(),
  on_game_boot_ready: (input: OnGameBootReadyEventData) =>
    new OnGameBootReady(input),
  startup: () => new Startup(),
}; // eventsConstructors close
