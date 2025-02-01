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
use crate::ecs_components::ui::ui_box_component::UIRectangleComponent;
use crate::ecs_components::ui::ui_color_component::UIColorComponent;
use crate::ecs_components::ui::ui_cursor_component::UICursorComponent;
use crate::ecs_components::ui::ui_hover_color_component::UIHoverColorComponent;
use crate::ecs_components::ui::ui_hover_cursor_component::UIHoverCursorComponent;
use crate::ecs_components::ui::ui_text_component::UITextComponent;
use crate::ecs_components::ui::ui_texture_component::UITextureComponent;
use crate::ecs_components::ui::ui_transform_component::UITransformComponent;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static COMPONENT_SEQ: AtomicU64 = AtomicU64::new(1);

pub trait ComponentTrait {
    fn id(&self) -> u64;
    fn get_type(&self) -> Components;
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

pub fn acquire_next_id() -> u64 {
    COMPONENT_SEQ.fetch_add(1, Ordering::SeqCst)
}

macro_rules! impl_component {
    ($type:ident, $type_short:ident) => {
        impl ComponentTrait for $type {
            fn id(&self) -> u64 {
                self.id
            }

            fn get_type(&self) -> Components {
                Components::$type_short
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
    };
}

macro_rules! create_component_types_enum {
    ($(($component_snake:ident, $component_short:ident, $component:ident, $component_multiple:ident)),+) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Components {
            $(
                $component_short,
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

            pub fn has(&self, typ: &Components) -> bool {
                match typ {
                    $(
                        Components::$component_short => has_component!(self, $component_snake, $component, $component_multiple),
                    )*
                }
            }

            pub fn has_all(&self, types: &[&Components]) -> bool {
                for typ in types {
                    if !self.has(typ) {
                        return false;
                    }
                }
                true
            }
        }

        $(
            impl_component!($component, $component_short);
        )*

    }
}

create_component_types_enum!(
    (camera_focus, CameraFocus, CameraFocusComponent, false),
    (
        first_person_camera_control,
        FirstPersonCameraControl,
        FirstPersonCameraControlComponent,
        false
    ),
    (
        third_person_orbit_camera_control,
        ThirdPersonOrbitCameraControl,
        ThirdPersonOrbitCameraControlComponent,
        false
    ),
    (
        third_person_static_camera_control,
        ThirdPersonStaticCameraControl,
        ThirdPersonStaticCameraControlComponent,
        false
    ),
    (transform, Transform, TransformComponent, false),
    (
        is_ground_collider,
        IsGroundCollider,
        IsGroundColliderComponent,
        false
    ),
    (real_physics, RealPhysics, RealPhysicsComponent, false),
    (simple_physics, SimplePhysics, SimplePhysicsComponent, false),
    (
        set_physics_kinematics,
        SetPhysicsKinematics,
        SetPhysicsKinematicsComponent,
        true
    ),
    (is_player, IsPlayer, IsPlayerComponent, false),
    (mesh, Mesh, MeshComponent, true),
    (control_focus, ControlFocus, ControlFocusComponent, false),
    (ship_control, ShipControl, ShipControlComponent, false),
    (ui_transform, UITransform, UITransformComponent, false),
    (ui_color, UIColor, UIColorComponent, false),
    (ui_hover_color, UIHoverColor, UIHoverColorComponent, false),
    (ui_rectangle, UIRectangle, UIRectangleComponent, false),
    (ui_cursor, UICursor, UICursorComponent, false),
    (
        ui_hover_cursor,
        UIHoverCursor,
        UIHoverCursorComponent,
        false
    ),
    (ui_texture, UITexture, UITextureComponent, false),
    (ui_text, UIText, UITextComponent, false)
);
