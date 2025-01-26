use crate::ecs_components::camera::camera_focus_component::CameraFocusComponent;
use crate::ecs_components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use crate::ecs_components::camera::third_person_orbit_camera_control_component::ThirdPersonOrbitCameraControlComponent;
use crate::ecs_components::camera::third_person_static_camera_control_component::ThirdPersonStaticCameraControlComponent;
use crate::ecs_components::common::control_focus_component::ControlFocusComponent;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::ecs_components::physics::is_ground_collider_component::IsGroundColliderComponent;
use crate::ecs_components::physics::real_physics_component::RealPhysicsComponent;
use crate::ecs_components::physics::simple_physics_component::SimplePhysicsComponent;
use crate::ecs_components::player::is_player_component::IsPlayerComponent;
use crate::ecs_components::rendering::mesh_component::MeshComponent;
use crate::ecs_components::ship::ship_control_component::ShipControlComponent;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use strum_macros::EnumDiscriminants;

static COMPONENT_SEQ: AtomicU64 = AtomicU64::new(1);

pub trait ComponentTrait {
    fn id(&self) -> u64;
    fn allow_multiple(&self) -> bool;
    fn as_enum(self) -> ComponentsEnum;
}

pub fn acquire_next_id() -> u64 {
    COMPONENT_SEQ.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(name(ComponentTypes))]
#[strum_discriminants(derive(Hash))]
pub enum ComponentsEnum {
    CameraFocusComponent(CameraFocusComponent),
    FirstPersonCameraControlComponent(FirstPersonCameraControlComponent),
    ThirdPersonOrbitCameraControlComponent(ThirdPersonOrbitCameraControlComponent),
    ThirdPersonStaticCameraControlComponent(ThirdPersonStaticCameraControlComponent),

    ControlFocusComponent(ControlFocusComponent),
    TransformComponent(TransformComponent),

    IsGroundColliderComponent(IsGroundColliderComponent),
    RealPhysicsComponent(RealPhysicsComponent),
    SimplePhysicsComponent(SimplePhysicsComponent),

    IsPlayerComponent(IsPlayerComponent),

    MeshComponent(MeshComponent),

    ShipControlComponent(ShipControlComponent),
}

impl ComponentsEnum {
    pub fn id(&self) -> u64 {
        match self {
            ComponentsEnum::CameraFocusComponent(c) => c.id(),
            ComponentsEnum::FirstPersonCameraControlComponent(c) => c.id(),
            ComponentsEnum::ThirdPersonOrbitCameraControlComponent(c) => c.id(),
            ComponentsEnum::ThirdPersonStaticCameraControlComponent(c) => c.id(),
            ComponentsEnum::ControlFocusComponent(c) => c.id(),
            ComponentsEnum::TransformComponent(c) => c.id(),
            ComponentsEnum::IsGroundColliderComponent(c) => c.id(),
            ComponentsEnum::RealPhysicsComponent(c) => c.id(),
            ComponentsEnum::SimplePhysicsComponent(c) => c.id(),
            ComponentsEnum::IsPlayerComponent(c) => c.id(),
            ComponentsEnum::MeshComponent(c) => c.id(),
            ComponentsEnum::ShipControlComponent(c) => c.id(),
        }
    }

    pub fn allow_multiple(&self) -> bool {
        match self {
            ComponentsEnum::CameraFocusComponent(c) => c.allow_multiple(),
            ComponentsEnum::FirstPersonCameraControlComponent(c) => c.allow_multiple(),
            ComponentsEnum::ThirdPersonOrbitCameraControlComponent(c) => c.allow_multiple(),
            ComponentsEnum::ThirdPersonStaticCameraControlComponent(c) => c.allow_multiple(),
            ComponentsEnum::ControlFocusComponent(c) => c.allow_multiple(),
            ComponentsEnum::TransformComponent(c) => c.allow_multiple(),
            ComponentsEnum::IsGroundColliderComponent(c) => c.allow_multiple(),
            ComponentsEnum::RealPhysicsComponent(c) => c.allow_multiple(),
            ComponentsEnum::SimplePhysicsComponent(c) => c.allow_multiple(),
            ComponentsEnum::IsPlayerComponent(c) => c.allow_multiple(),
            ComponentsEnum::MeshComponent(c) => c.allow_multiple(),
            ComponentsEnum::ShipControlComponent(c) => c.allow_multiple(),
        }
    }

    pub fn typ(&self) -> ComponentTypes {
        match self {
            ComponentsEnum::CameraFocusComponent(_) => ComponentTypes::CameraFocusComponent,
            ComponentsEnum::FirstPersonCameraControlComponent(_) => {
                ComponentTypes::FirstPersonCameraControlComponent
            }
            ComponentsEnum::ThirdPersonOrbitCameraControlComponent(_) => {
                ComponentTypes::ThirdPersonOrbitCameraControlComponent
            }
            ComponentsEnum::ThirdPersonStaticCameraControlComponent(_) => {
                ComponentTypes::ThirdPersonStaticCameraControlComponent
            }
            ComponentsEnum::ControlFocusComponent(_) => ComponentTypes::ControlFocusComponent,
            ComponentsEnum::TransformComponent(_) => ComponentTypes::TransformComponent,
            ComponentsEnum::IsGroundColliderComponent(_) => {
                ComponentTypes::IsGroundColliderComponent
            }
            ComponentsEnum::RealPhysicsComponent(_) => ComponentTypes::RealPhysicsComponent,
            ComponentsEnum::SimplePhysicsComponent(_) => ComponentTypes::SimplePhysicsComponent,
            ComponentsEnum::IsPlayerComponent(_) => ComponentTypes::IsPlayerComponent,
            ComponentsEnum::MeshComponent(_) => ComponentTypes::MeshComponent,
            ComponentsEnum::ShipControlComponent(_) => ComponentTypes::ShipControlComponent,
        }
    }
}

#[macro_export]
macro_rules! impl_component {
    ($type:ident, $allow_multiple:expr) => {
        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn allow_multiple(&self) -> bool {
                $allow_multiple
            }

            fn as_enum(self) -> ComponentsEnum {
                ComponentsEnum::$type(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_marker_component {
    ($type:ident, $allow_multiple:expr) => {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct $type {
            pub id: u64,
        }

        impl $type {
            pub fn new() -> Self {
                Self {
                    id: acquire_next_id(),
                }
            }
        }

        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn allow_multiple(&self) -> bool {
                $allow_multiple
            }

            fn as_enum(&self) -> ComponentsEnum {
                ComponentsEnum::$type(self)
            }
        }
    };
}

#[macro_export]
macro_rules! component_types {
    ($($component:ty),+) => {
        &[$(
            & < $component > ::typ(),
        )*]
    };
}
