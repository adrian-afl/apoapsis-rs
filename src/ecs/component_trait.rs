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
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicU64, Ordering};

static COMPONENT_SEQ: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentTypes {
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

pub trait ComponentTrait: Any {
    fn id(&self) -> u64;
    fn allow_multiple(&self) -> bool;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
    fn as_component_enum(&self) -> ComponentTypes;
    fn typ(&self) -> TypeId;
}

pub fn acquire_next_id() -> u64 {
    COMPONENT_SEQ.fetch_add(1, Ordering::SeqCst)
}

pub fn component_type<T: ComponentTrait>() -> TypeId {
    TypeId::of::<T>()
}

#[macro_export]
macro_rules! component_from_enum {
    ($enum:ident, $type:ident) => {
        match $enum {
            ComponentTypes::$type(x) => x,
            _ => panic!("Failed to convert component from enum"),
        }
    };
}

#[macro_export]
macro_rules! component_trait_from_enum {
    ($enum:ident) => {
        match $enum {
            ComponentTypes::CameraFocusComponent(x) => Box::new(x),
            ComponentTypes::FirstPersonCameraControlComponent(x) => Box::new(x),
            ComponentTypes::ThirdPersonOrbitCameraControlComponent(x) => Box::new(x),
            ComponentTypes::ThirdPersonStaticCameraControlComponent(x) => Box::new(x),
            ComponentTypes::ControlFocusComponent(x) => Box::new(x),
            ComponentTypes::TransformComponent(x) => Box::new(x),
            ComponentTypes::IsGroundColliderComponent(x) => Box::new(x),
            ComponentTypes::RealPhysicsComponent(x) => Box::new(x),
            ComponentTypes::SimplePhysicsComponent(x) => Box::new(x),
            ComponentTypes::IsPlayerComponent(x) => Box::new(x),
            ComponentTypes::MeshComponent(x) => Box::new(x),
            ComponentTypes::ShipControlComponent(x) => Box::new(x),
        }
    };
}

#[macro_export]
macro_rules! impl_component {
    ($type:ident, $allow_multiple:expr) => {
        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn typ(&self) -> TypeId {
                component_type::<$type>()
            }

            fn allow_multiple(&self) -> bool {
                $allow_multiple
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn as_component_enum(&self) -> ComponentTypes {
                ComponentTypes::$type(self.clone())
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

            fn typ(&self) -> TypeId {
                component_type::<$type>()
            }

            fn allow_multiple(&self) -> bool {
                $allow_multiple
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn as_component_enum(&self) -> ComponentTypes {
                ComponentTypes::$type(self.clone())
            }
        }
    };
}

#[macro_export]
macro_rules! component_types {
    ($($component:ty),+) => {
        &[$(
            &component_type::<$component>(),
        )*]
    };
}
