use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use glam::{DQuat, DVec3};
use std::any::{Any, TypeId};

#[derive(Clone, Debug)]
pub enum OrbitCameraStyle {
    Absolute,
    RelativeToEntity,
    RelativeToSurface,
}

#[derive(Clone, Debug)]
pub struct ThirdPersonOrbitCameraControlComponent {
    pub id: u64,
    pub initial_offset: DVec3,
    pub initial_orientation: DQuat,
    pub style: OrbitCameraStyle,
}

impl_component!(ThirdPersonOrbitCameraControlComponent, false);

impl ThirdPersonOrbitCameraControlComponent {
    pub fn new(initial_offset: DVec3, initial_orientation: DQuat, style: OrbitCameraStyle) -> Self {
        Self {
            id: acquire_next_id(),
            initial_offset,
            initial_orientation,
            style,
        }
    }
}
