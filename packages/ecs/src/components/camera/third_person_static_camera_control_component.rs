use crate::component_trait::acquire_next_id;
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct ThirdPersonStaticCameraControlComponent {
    #[serde(skip, default = "acquire_next_id")]
    pub id: u64,
    pub fov: f64,
    #[ts(type = "[number, number, number]")]
    pub offset: DVec3,
    #[ts(type = "[number, number, number, number]")]
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
