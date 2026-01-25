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
import { ControlMapItem } from "./types/ControlMapItem.js";
import { OnPhysicsCollisionEventData } from "./types/OnPhysicsCollisionEventData.js";
import { OnGameBootReadyEventData } from "./types/OnGameBootReadyEventData.js";

export abstract class AbstractBaseEvent {
  public eventName = this.constructor.name;
}

export class OnControlActivate extends AbstractBaseEvent {
  public readonly data: ControlMapItem;
  public constructor(input: ControlMapItem) {
    super();
    this.data = input;
  }
}

export class OnControlRelease extends AbstractBaseEvent {
  public readonly data: ControlMapItem;
  public constructor(input: ControlMapItem) {
    super();
    this.data = input;
  }
}

export class OnRawKeyDown extends AbstractBaseEvent {
  public readonly data: number;
  public constructor(input: number) {
    super();
    this.data = input;
  }
}

export class OnRawKeyUp extends AbstractBaseEvent {
  public readonly data: number;
  public constructor(input: number) {
    super();
    this.data = input;
  }
}

export class OnRawInputText extends AbstractBaseEvent {
  public readonly data: string;
  public constructor(input: string) {
    super();
    this.data = input;
  }
}

export class OnRemoteGameModeInitialized extends AbstractBaseEvent {}

export class OnPhysicsCollisionEvent extends AbstractBaseEvent {
  public readonly data: OnPhysicsCollisionEventData;
  public constructor(input: OnPhysicsCollisionEventData) {
    super();
    this.data = input;
  }
}

export class OnGameBootReady extends AbstractBaseEvent {
  public readonly data: OnGameBootReadyEventData;
  public constructor(input: OnGameBootReadyEventData) {
    super();
    this.data = input;
  }
}

export class Startup extends AbstractBaseEvent {}

export const eventsConstructors = {
  on_control_activate: (input: ControlMapItem) => new OnControlActivate(input),
  on_control_release: (input: ControlMapItem) => new OnControlRelease(input),
  on_raw_key_down: (input: number) => new OnRawKeyDown(input),
  on_raw_key_up: (input: number) => new OnRawKeyUp(input),
  on_raw_input_text: (input: string) => new OnRawInputText(input),
  on_remote_game_mode_initialized: () => new OnRemoteGameModeInitialized(),
  on_physics_collision_event: (input: OnPhysicsCollisionEventData) =>
    new OnPhysicsCollisionEvent(input),
  on_game_boot_ready: (input: OnGameBootReadyEventData) =>
    new OnGameBootReady(input),
  startup: () => new Startup(),
}; // eventsConstructors close
