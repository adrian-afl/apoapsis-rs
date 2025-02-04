use crate::component_trait::acquire_next_id;
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OrbitCameraStyle {
    Absolute,
    RelativeToEntity,
    RelativeToSurface,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThirdPersonOrbitCameraControlComponent {
    pub id: u64,
    pub fov: f64,
    pub initial_offset: DVec3,
    pub initial_orientation: DQuat,
    pub style: OrbitCameraStyle,
}

impl ThirdPersonOrbitCameraControlComponent {
    pub fn new(
        fov: f64,
        initial_offset: DVec3,
        initial_orientation: DQuat,
        style: OrbitCameraStyle,
    ) -> Self {
        Self {
            id: acquire_next_id(),
            fov,
            initial_offset,
            initial_orientation,
            style,
        }
    }
}
