use crate::component_trait::acquire_next_id;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct FirstPersonCameraControlComponent {
    #[serde(skip, default = "acquire_next_id")]
    pub id: u64,
    pub fov: f64,
}

impl FirstPersonCameraControlComponent {
    pub fn new(fov: f64) -> Self {
        Self {
            id: acquire_next_id(),
            fov,
        }
    }
}
