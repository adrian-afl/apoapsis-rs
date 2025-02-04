use crate::component_trait::acquire_next_id;
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThirdPersonStaticCameraControlComponent {
    pub id: u64,
    pub fov: f64,
    pub offset: DVec3,
    pub orientation: DQuat,
}

impl ThirdPersonStaticCameraControlComponent {
    pub fn new(fov: f64, offset: DVec3, orientation: DQuat) -> Self {
        Self {
            id: acquire_next_id(),
            fov,
            offset,
            orientation,
        }
    }
}
