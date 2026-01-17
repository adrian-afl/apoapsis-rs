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
import { ECSWorldSerializedRepresentation } from "./types/ECSWorldSerializedRepresentation.js";
import { AddEntityInput } from "./types/AddEntityInput.js";
import { ObjectWithID } from "./types/ObjectWithID.js";
import { Entity } from "./types/Entity.js";
import { AttachedComponents } from "./types/AttachedComponents.js";
import { DecimalVector3d } from "./types/DecimalVector3d.js";
import { BodyCelestialBodyDefinition } from "./types/BodyCelestialBodyDefinition.js";
import { DecimalMatrix3d } from "./types/DecimalMatrix3d.js";
import { BaseGameApi } from "../framework/BaseGameApi.js";

export class RemoteGameApi {
  private readonly api: BaseGameApi;

  public constructor(api: BaseGameApi) {
    this.api = api;
  }

  public async deserializeWorld(
    serializedWorld: ECSWorldSerializedRepresentation,
  ): Promise<void> {
    return this.api.send({
      name: "command.deserialize_world",
      payload: serializedWorld,
    }) as Promise<void>;
  }

  public async addEntity(input: AddEntityInput): Promise<ObjectWithID> {
    return this.api.send({
      name: "command.add_entity",
      payload: input,
    }) as Promise<ObjectWithID>;
  }

  public async removeEntity(id: number): Promise<void> {
    return this.api.send({
      name: "command.remove_entity",
      payload: id,
    }) as Promise<void>;
  }

  public async getEntity(id: number): Promise<Entity> {
    return this.api.send({
      name: "command.get_entity",
      payload: id,
    }) as Promise<Entity>;
  }

  public async replaceEntityComponents(
    id: number,
    components: AttachedComponents,
  ): Promise<void> {
    return this.api.send({
      name: "command.replace_entity_components",
      payload: { id, components },
    }) as Promise<void>;
  }

  public async findAllEntitiesByComponents(
    components: AttachedComponents[],
  ): Promise<number[]> {
    return this.api.send({
      name: "command.find_all_entities_by_components",
      payload: components,
    }) as Promise<number[]>;
  }

  public async resetWorld(): Promise<void> {
    return this.api.send({
      name: "command.reset_world",
      payload: {},
    }) as Promise<void>;
  }

  public async serializeWorld(): Promise<ECSWorldSerializedRepresentation> {
    return this.api.send({
      name: "command.serialize_world",
      payload: {},
    }) as Promise<ECSWorldSerializedRepresentation>;
  }

  public async getAllCelestialBodyNames(): Promise<string[]> {
    return this.api.send({
      name: "command.get_all_celestial_body_names",
      payload: {},
    }) as Promise<string[]>;
  }

  public async getCelestialBodyPosition(
    name: string,
  ): Promise<DecimalVector3d> {
    return this.api.send({
      name: "command.get_celestial_body_position",
      payload: name,
    }) as Promise<DecimalVector3d>;
  }

  public async getCelestialBodyDefinition(
    name: string,
  ): Promise<BodyCelestialBodyDefinition> {
    return this.api.send({
      name: "command.get_celestial_body_definition",
      payload: name,
    }) as Promise<BodyCelestialBodyDefinition>;
  }

  public async getCelestialBodySurfaceVelocity(
    name: string,
    point: DecimalVector3d,
  ): Promise<DecimalVector3d> {
    return this.api.send({
      name: "command.get_celestial_body_surface_velocity",
      payload: { name, point },
    }) as Promise<DecimalVector3d>;
  }

  public async getCelestialBodyOrientation(
    name: string,
  ): Promise<DecimalMatrix3d> {
    return this.api.send({
      name: "command.get_celestial_body_orientation",
      payload: name,
    }) as Promise<DecimalMatrix3d>;
  }

  public async getCelestialBodyParent(name: string): Promise<string | null> {
    return this.api.send({
      name: "command.get_celestial_body_parent",
      payload: name,
    }) as Promise<string | null>;
  }

  public async getCelestialBodySatellites(name: string): Promise<string[]> {
    return this.api.send({
      name: "command.get_celestial_body_satellites",
      payload: name,
    }) as Promise<string[]>;
  }

  public async getAltitudeOverCelestialBody(
    name: string,
    point: DecimalVector3d,
  ): Promise<string> {
    return this.api.send({
      name: "command.get_altitude_over_celestial_body",
      payload: { name, point },
    }) as Promise<string>;
  }

  public async getClosestCelestialBody(
    point: DecimalVector3d,
  ): Promise<string> {
    return this.api.send({
      name: "command.get_closest_celestial_body",
      payload: point,
    }) as Promise<string>;
  }

  public async getGravityFlux(
    point: DecimalVector3d,
  ): Promise<DecimalVector3d> {
    return this.api.send({
      name: "command.get_gravity_flux",
      payload: point,
    }) as Promise<DecimalVector3d>;
  }

  public universeClock = {
    get: (entityId: number): Promise<UniverseClockComponent> => {
      return this.api.send({
        name: "command.get_universe_clock",
        payload: { entityId },
      }) as Promise<UniverseClockComponent>;
    },

    set: async (
      entityId: number,
      component: UniverseClockComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_universe_clock",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_universe_clock",
        payload: { entityId },
      });
    },
  };

  public firstPersonCameraControl = {
    get: (entityId: number): Promise<FirstPersonCameraControlComponent> => {
      return this.api.send({
        name: "command.get_first_person_camera_control",
        payload: { entityId },
      }) as Promise<FirstPersonCameraControlComponent>;
    },

    set: async (
      entityId: number,
      component: FirstPersonCameraControlComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_first_person_camera_control",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_first_person_camera_control",
        payload: { entityId },
      });
    },
  };

  public thirdPersonOrbitCameraControl = {
    get: (
      entityId: number,
    ): Promise<ThirdPersonOrbitCameraControlComponent> => {
      return this.api.send({
        name: "command.get_third_person_orbit_camera_control",
        payload: { entityId },
      }) as Promise<ThirdPersonOrbitCameraControlComponent>;
    },

    set: async (
      entityId: number,
      component: ThirdPersonOrbitCameraControlComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_third_person_orbit_camera_control",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_third_person_orbit_camera_control",
        payload: { entityId },
      });
    },
  };

  public thirdPersonStaticCameraControl = {
    get: (
      entityId: number,
    ): Promise<ThirdPersonStaticCameraControlComponent> => {
      return this.api.send({
        name: "command.get_third_person_static_camera_control",
        payload: { entityId },
      }) as Promise<ThirdPersonStaticCameraControlComponent>;
    },

    set: async (
      entityId: number,
      component: ThirdPersonStaticCameraControlComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_third_person_static_camera_control",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_third_person_static_camera_control",
        payload: { entityId },
      });
    },
  };

  public transform = {
    get: (entityId: number): Promise<TransformComponent> => {
      return this.api.send({
        name: "command.get_transform",
        payload: { entityId },
      }) as Promise<TransformComponent>;
    },

    set: async (
      entityId: number,
      component: TransformComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_transform",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_transform",
        payload: { entityId },
      });
    },
  };

  public realPhysics = {
    get: (entityId: number): Promise<RealPhysicsComponent> => {
      return this.api.send({
        name: "command.get_real_physics",
        payload: { entityId },
      }) as Promise<RealPhysicsComponent>;
    },

    set: async (
      entityId: number,
      component: RealPhysicsComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_real_physics",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_real_physics",
        payload: { entityId },
      });
    },
  };

  public simplePhysics = {
    get: (entityId: number): Promise<SimplePhysicsComponent> => {
      return this.api.send({
        name: "command.get_simple_physics",
        payload: { entityId },
      }) as Promise<SimplePhysicsComponent>;
    },

    set: async (
      entityId: number,
      component: SimplePhysicsComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_simple_physics",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_simple_physics",
        payload: { entityId },
      });
    },
  };

  public shipControl = {
    get: (entityId: number): Promise<ShipControlComponent> => {
      return this.api.send({
        name: "command.get_ship_control",
        payload: { entityId },
      }) as Promise<ShipControlComponent>;
    },

    set: async (
      entityId: number,
      component: ShipControlComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_ship_control",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_ship_control",
        payload: { entityId },
      });
    },
  };

  public uIColor = {
    get: (entityId: number): Promise<UIColorComponent> => {
      return this.api.send({
        name: "command.get_ui_color",
        payload: { entityId },
      }) as Promise<UIColorComponent>;
    },

    set: async (
      entityId: number,
      component: UIColorComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_ui_color",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_ui_color",
        payload: { entityId },
      });
    },
  };

  public uIHoverColor = {
    get: (entityId: number): Promise<UIHoverColorComponent> => {
      return this.api.send({
        name: "command.get_ui_hover_color",
        payload: { entityId },
      }) as Promise<UIHoverColorComponent>;
    },

    set: async (
      entityId: number,
      component: UIHoverColorComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_ui_hover_color",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_ui_hover_color",
        payload: { entityId },
      });
    },
  };

  public uIBox = {
    get: (entityId: number): Promise<UIBoxComponent> => {
      return this.api.send({
        name: "command.get_ui_box",
        payload: { entityId },
      }) as Promise<UIBoxComponent>;
    },

    set: async (entityId: number, component: UIBoxComponent): Promise<void> => {
      await this.api.send({
        name: "command.set_ui_box",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_ui_box",
        payload: { entityId },
      });
    },
  };

  public uIHoverCursor = {
    get: (entityId: number): Promise<UIHoverCursorComponent> => {
      return this.api.send({
        name: "command.get_ui_hover_cursor",
        payload: { entityId },
      }) as Promise<UIHoverCursorComponent>;
    },

    set: async (
      entityId: number,
      component: UIHoverCursorComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_ui_hover_cursor",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_ui_hover_cursor",
        payload: { entityId },
      });
    },
  };

  public uITexture = {
    get: (entityId: number): Promise<UITextureComponent> => {
      return this.api.send({
        name: "command.get_ui_texture",
        payload: { entityId },
      }) as Promise<UITextureComponent>;
    },

    set: async (
      entityId: number,
      component: UITextureComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_ui_texture",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_ui_texture",
        payload: { entityId },
      });
    },
  };

  public uIText = {
    get: (entityId: number): Promise<UITextComponent> => {
      return this.api.send({
        name: "command.get_ui_text",
        payload: { entityId },
      }) as Promise<UITextComponent>;
    },

    set: async (
      entityId: number,
      component: UITextComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.set_ui_text",
        payload: { entityId, component },
      });
    },

    clear: async (entityId: number): Promise<void> => {
      await this.api.send({
        name: "command.clear_ui_text",
        payload: { entityId },
      });
    },
  };

  public setPhysicsKinematics = {
    get: (entityId: number): Promise<SetPhysicsKinematicsComponent[]> => {
      return this.api.send({
        name: "command.get_set_physics_kinematics",
        payload: { entityId },
      }) as Promise<SetPhysicsKinematicsComponent[]>;
    },

    add: async (
      entityId: number,
      component: SetPhysicsKinematicsComponent,
    ): Promise<void> => {
      await this.api.send({
        name: "command.add_set_physics_kinematics",
        payload: { entityId, component },
      });
    },

    remove: async (entityId: number, componentId: number): Promise<void> => {
      await this.api.send({
        name: "command.remove_set_physics_kinematics",
        payload: { entityId, componentId },
      });
    },
  };

  public mesh = {
    get: (entityId: number): Promise<MeshComponent[]> => {
      return this.api.send({
        name: "command.get_mesh",
        payload: { entityId },
      }) as Promise<MeshComponent[]>;
    },

    add: async (entityId: number, component: MeshComponent): Promise<void> => {
      await this.api.send({
        name: "command.add_mesh",
        payload: { entityId, component },
      });
    },

    remove: async (entityId: number, componentId: number): Promise<void> => {
      await this.api.send({
        name: "command.remove_mesh",
        payload: { entityId, componentId },
      });
    },
  };

  public cameraFocus = {
    get: (entityId: number): Promise<CameraFocusComponent> => {
      return this.api.send({
        name: "command.get_camera_focus",
        payload: { entityId },
      }) as Promise<CameraFocusComponent>;
    },

    set: async (entityId: number, value: boolean): Promise<void> => {
      await this.api.send({
        name: "command.set_camera_focus",
        payload: { entityId, value },
      });
    },
  };

  public isGroundCollider = {
    get: (entityId: number): Promise<IsGroundColliderComponent> => {
      return this.api.send({
        name: "command.get_is_ground_collider",
        payload: { entityId },
      }) as Promise<IsGroundColliderComponent>;
    },

    set: async (entityId: number, value: boolean): Promise<void> => {
      await this.api.send({
        name: "command.set_is_ground_collider",
        payload: { entityId, value },
      });
    },
  };

  public isPlayer = {
    get: (entityId: number): Promise<IsPlayerComponent> => {
      return this.api.send({
        name: "command.get_is_player",
        payload: { entityId },
      }) as Promise<IsPlayerComponent>;
    },

    set: async (entityId: number, value: boolean): Promise<void> => {
      await this.api.send({
        name: "command.set_is_player",
        payload: { entityId, value },
      });
    },
  };

  public controlFocus = {
    get: (entityId: number): Promise<ControlFocusComponent> => {
      return this.api.send({
        name: "command.get_control_focus",
        payload: { entityId },
      }) as Promise<ControlFocusComponent>;
    },

    set: async (entityId: number, value: boolean): Promise<void> => {
      await this.api.send({
        name: "command.set_control_focus",
        payload: { entityId, value },
      });
    },
  };

  public uIIsRaycastable = {
    get: (entityId: number): Promise<UIIsRaycastableComponent> => {
      return this.api.send({
        name: "command.get_ui_is_raycastable",
        payload: { entityId },
      }) as Promise<UIIsRaycastableComponent>;
    },

    set: async (entityId: number, value: boolean): Promise<void> => {
      await this.api.send({
        name: "command.set_ui_is_raycastable",
        payload: { entityId, value },
      });
    },
  };

  public uIRequireFreeCursor = {
    get: (entityId: number): Promise<UIRequireFreeCursorComponent> => {
      return this.api.send({
        name: "command.get_ui_require_free_cursor",
        payload: { entityId },
      }) as Promise<UIRequireFreeCursorComponent>;
    },

    set: async (entityId: number, value: boolean): Promise<void> => {
      await this.api.send({
        name: "command.set_ui_require_free_cursor",
        payload: { entityId, value },
      });
    },
  };
}
export const emptyAttachedComponents = {
  universe_clock: null,
  camera_focus: false,
  first_person_camera_control: null,
  third_person_orbit_camera_control: null,
  third_person_static_camera_control: null,
  transform: null,
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
};
