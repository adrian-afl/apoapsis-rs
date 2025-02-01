use crate::ecs_components::camera::camera_focus_component::CameraFocusComponent;
use crate::ecs_components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use crate::ecs_components::camera::third_person_orbit_camera_control_component::ThirdPersonOrbitCameraControlComponent;
use crate::ecs_components::camera::third_person_static_camera_control_component::ThirdPersonStaticCameraControlComponent;
use crate::ecs_components::common::control_focus_component::ControlFocusComponent;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::ecs_components::physics::is_ground_collider_component::IsGroundColliderComponent;
use crate::ecs_components::physics::real_physics_component::RealPhysicsComponent;
use crate::ecs_components::physics::set_physics_kinematics_component::SetPhysicsKinematicsComponent;
use crate::ecs_components::physics::simple_physics_component::SimplePhysicsComponent;
use crate::ecs_components::player::is_player_component::IsPlayerComponent;
use crate::ecs_components::rendering::mesh_component::MeshComponent;
use crate::ecs_components::ship::ship_control_component::ShipControlComponent;
use crate::ecs_components::ui::ui_element_component::UIElementComponent;
use serde::{Deserialize, Serialize};

use crate::ecs_components::ui::ui_color_component::UIColorComponent;
use crate::ecs_components::ui::ui_cursor_component::UICursorComponent;
use crate::ecs_components::ui::ui_hover_color_component::UIHoverColorComponent;
use crate::ecs_components::ui::ui_hover_cursor_component::UIHoverCursorComponent;
use crate::ecs_components::ui::ui_rectangle_component::UIRectangleComponent;
use crate::ecs_components::ui::ui_text_component::UITextComponent;
use crate::ecs_components::ui::ui_texture_component::UITextureComponent;
use crate::ecs_components::ui::ui_transform_component::UITransformComponent;
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicU64, Ordering};

static COMPONENT_SEQ: AtomicU64 = AtomicU64::new(1);

pub trait ComponentTrait: Any {
    fn id(&self) -> u64;
    fn get_type(&self) -> ComponentTypes;
}

macro_rules! vector_or_option_type {
    ($component_type:ident, true) => {
        Vec< $component_type >
    };
    ($component_type:ident, false) => {
        Option< $component_type >
    };
}

macro_rules! vector_or_option_initializer {
    ($component_type:ident, true) => {
        Vec::new()
    };
    ($component_type:ident, false) => {
        None
    };
}

macro_rules! has_component {
    ($self:ident, $component_snake:ident, $component:ident, true) => {
        $self.$component_snake.len() > 0
    };
    ($self:ident, $component_snake:ident, $component:ident, false) => {
        $self.$component_snake.is_some()
    };
}

macro_rules! create_component_types_enum {
    ($(($component_snake:ident, $component:ident, $component_multiple:ident)),+) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum ComponentTypes {
            $(
                $component,
            )*
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AttachedComponents {
            $(
                pub $component_snake: vector_or_option_type!($component, $component_multiple),
            )*
        }

        impl AttachedComponents {
            pub fn new() -> Self {
                Self {
                    $(
                        $component_snake: vector_or_option_initializer!($component, $component_multiple),
                    )*
                }
            }

            pub fn has(&self, typ: &ComponentTypes) -> bool {
                match typ {
                    $(
                        ComponentTypes::$component => has_component!(self, $component_snake, $component, $component_multiple),
                    )*
                }
            }

            pub fn has_all(&self, types: &[&ComponentTypes]) -> bool {
                for typ in types {
                    if !self.has(typ) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

create_component_types_enum!(
    (camera_focus, CameraFocusComponent, false),
    (
        first_person_camera_control,
        FirstPersonCameraControlComponent,
        false
    ),
    (
        third_person_orbit_camera_control,
        ThirdPersonOrbitCameraControlComponent,
        false
    ),
    (
        third_person_static_camera_control,
        ThirdPersonStaticCameraControlComponent,
        false
    ),
    (transform, TransformComponent, false),
    (is_ground_collider, IsGroundColliderComponent, false),
    (real_physics, RealPhysicsComponent, false),
    (simple_physics, SimplePhysicsComponent, false),
    (set_physics_kinematics, SetPhysicsKinematicsComponent, true),
    (is_player, IsPlayerComponent, false),
    (mesh, MeshComponent, true),
    (control_focus, ControlFocusComponent, false),
    (ship_control, ShipControlComponent, false),
    (ui_transform, UITransformComponent, false),
    (ui_color, UIColorComponent, false),
    (ui_hover_color, UIHoverColorComponent, false),
    (ui_rectangle, UIRectangleComponent, false),
    (ui_cursor, UICursorComponent, false),
    (ui_hover_cursor, UIHoverCursorComponent, false),
    (ui_texture, UITextureComponent, false),
    (ui_text, UITextComponent, false)
);

pub fn acquire_next_id() -> u64 {
    COMPONENT_SEQ.fetch_add(1, Ordering::SeqCst)
}

pub fn component_type<T: ComponentTrait>() -> TypeId {
    TypeId::of::<T>()
}

#[macro_export]
macro_rules! impl_component {
    ($type:ident) => {
        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn get_type(&self) -> ComponentTypes {
                ComponentTypes::$type
            }
        }
    };
}

#[macro_export]
macro_rules! impl_marker_component {
    ($type:ident) => {
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

            fn get_type(&self) -> ComponentTypes {
                ComponentTypes::$type
            }
        }
    };
}
