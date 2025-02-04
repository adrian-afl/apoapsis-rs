use crate::component_trait::acquire_next_id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirstPersonCameraControlComponent {
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
