use crate::components::camera::camera_focus_component::CameraFocusComponent;
use crate::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use crate::components::camera::third_person_orbit_camera_control_component::ThirdPersonOrbitCameraControlComponent;
use crate::components::camera::third_person_static_camera_control_component::ThirdPersonStaticCameraControlComponent;
use crate::components::common::control_focus_component::ControlFocusComponent;
use crate::components::common::transform_component::TransformComponent;
use crate::components::common::universe_clock_component::UniverseClockComponent;
use crate::components::physics::glue_to_celestial_body_component::GlueToCelestialBodyComponent;
use crate::components::physics::is_celestial_body_surface_component::IsCelestialBodySurfaceComponent;
use crate::components::physics::is_ground_collider_component::IsGroundColliderComponent;
use crate::components::physics::real_physics_component::RealPhysicsComponent;
use crate::components::physics::set_physics_kinematics_component::SetPhysicsKinematicsComponent;
use crate::components::physics::simple_physics_component::SimplePhysicsComponent;
use crate::components::player::is_player_component::IsPlayerComponent;
use crate::components::rendering::mesh_component::MeshComponent;
use crate::components::ship::ship_control_component::ShipControlComponent;
use crate::components::ui::ui_box_component::UIBoxComponent;
use crate::components::ui::ui_color_component::UIColorComponent;
use crate::components::ui::ui_hover_color_component::UIHoverColorComponent;
use crate::components::ui::ui_hover_cursor_component::UIHoverCursorComponent;
use crate::components::ui::ui_is_raycastable_component::UIIsRaycastableComponent;
use crate::components::ui::ui_require_free_cursor_component::UIRequireFreeCursorComponent;
use crate::components::ui::ui_text_component::UITextComponent;
use crate::components::ui::ui_texture_component::UITextureComponent;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use ts_rs::TS;

static COMPONENT_SEQ: AtomicU64 = AtomicU64::new(1);

pub trait ComponentTrait {
    fn id(&self) -> u64;
    fn get_type(&self) -> Components;
}

macro_rules! vector_or_option_type {
    ($component_type:ident, Vector) => {
        Vec< $component_type >
    };
    ($component_type:ident, Option) => {
        Option< $component_type >
    };
    ($component_type:ident, Marker) => {
        bool
    };
}

macro_rules! vector_or_option_initializer {
    ($component_type:ident, Vector) => {
        Vec::new()
    };
    ($component_type:ident, Option) => {
        None
    };
    ($component_type:ident, Marker) => {
        false
    };
}

macro_rules! vector_or_option_id_regenerator {
    ($self:ident, $component_snake:ident, Vector) => {
        $self
            .$component_snake
            .iter_mut()
            .for_each(|x| x.id = acquire_next_id())
    };
    ($self:ident, $component_snake:ident, Option) => {{
        match &mut $self.$component_snake {
            Some(x) => x.id = acquire_next_id(),
            None => (),
        }
    }};
    ($self:ident, $component_snake:ident, Marker) => {};
}

macro_rules! has_component {
    ($self:ident, $component_snake:ident, $component:ident, Vector) => {
        $self.$component_snake.len() > 0
    };
    ($self:ident, $component_snake:ident, $component:ident, Option) => {
        $self.$component_snake.is_some()
    };
    ($self:ident, $component_snake:ident, $component:ident, Marker) => {
        $self.$component_snake
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
        #[derive(Clone, Debug, Serialize, Deserialize, TS)]
        #[ts(export)]
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
        #[derive(Debug, Clone, Serialize, Deserialize, TS)]
        #[ts(export)]
        pub enum Components {
            $(
                $component_short,
            )*
        }

        #[derive(Debug, Clone, Serialize, Deserialize, TS)]
        #[ts(export)]
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

            pub fn regenerate_ids(&mut self) {
                $(
                    vector_or_option_id_regenerator!(self, $component_snake, $component_multiple);
                )*
            }
        }

        $(
            impl_component!($component, $component_short);
        )*

    }
}

create_component_types_enum!(
    (
        universe_clock,
        UniverseClock,
        UniverseClockComponent,
        Option
    ),
    (camera_focus, CameraFocus, CameraFocusComponent, Marker),
    (
        first_person_camera_control,
        FirstPersonCameraControl,
        FirstPersonCameraControlComponent,
        Option
    ),
    (
        third_person_orbit_camera_control,
        ThirdPersonOrbitCameraControl,
        ThirdPersonOrbitCameraControlComponent,
        Option
    ),
    (
        third_person_static_camera_control,
        ThirdPersonStaticCameraControl,
        ThirdPersonStaticCameraControlComponent,
        Option
    ),
    (transform, Transform, TransformComponent, Option),
    (
        is_ground_collider,
        IsGroundCollider,
        IsGroundColliderComponent,
        Marker
    ),
    (real_physics, RealPhysics, RealPhysicsComponent, Option),
    (
        simple_physics,
        SimplePhysics,
        SimplePhysicsComponent,
        Option
    ),
    (
        set_physics_kinematics,
        SetPhysicsKinematics,
        SetPhysicsKinematicsComponent,
        Vector
    ),
    (
        glue_to_celestial_body,
        GlueToCelestialBody,
        GlueToCelestialBodyComponent,
        Option
    ),
    (
        is_celestial_body_surface,
        IsCelestialBodySurface,
        IsCelestialBodySurfaceComponent,
        Marker
    ),
    (is_player, IsPlayer, IsPlayerComponent, Marker),
    (mesh, Mesh, MeshComponent, Vector),
    (control_focus, ControlFocus, ControlFocusComponent, Marker),
    (ship_control, ShipControl, ShipControlComponent, Option),
    (ui_color, UIColor, UIColorComponent, Option),
    (ui_hover_color, UIHoverColor, UIHoverColorComponent, Option),
    (ui_box, UIBox, UIBoxComponent, Option),
    (
        ui_hover_cursor,
        UIHoverCursor,
        UIHoverCursorComponent,
        Option
    ),
    (ui_texture, UITexture, UITextureComponent, Option),
    (ui_text, UIText, UITextComponent, Option),
    (
        ui_is_raycastable,
        UIIsRaycastable,
        UIIsRaycastableComponent,
        Marker
    ),
    (
        ui_require_free_cursor,
        UIRequireFreeCursor,
        UIRequireFreeCursorComponent,
        Marker
    )
);
