use crate::component_trait::acquire_next_id;
use glam::{DQuat, DVec2, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct UIBoxComponent {
    pub id: u64,
    #[ts(type = "[number, number]")]
    pub size: DVec2,
    #[ts(type = "[number, number]")]
    pub position: DVec2,
    pub orientation: f64, // radians
    pub z_index: i32,
}

impl UIBoxComponent {
    pub fn default() -> Self {
        Self {
            id: acquire_next_id(),
            size: DVec2::new(0.0, 0.0),
            position: DVec2::new(0.0, 0.0),
            orientation: 0.0,
            z_index: 0,
        }
    }

    pub fn with_position(mut self, position: DVec2) -> Self {
        self.position = position;
        self
    }

    pub fn with_size(mut self, size: DVec2) -> Self {
        self.size = size;
        self
    }

    pub fn with_orientation(mut self, orientation: f64) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }
}
