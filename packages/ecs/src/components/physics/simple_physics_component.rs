use crate::component_trait::acquire_next_id;
use glam::DVec3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct SimplePhysicsComponent {
    #[serde(skip, default = "acquire_next_id")]
    pub id: u64,
    pub is_static: bool,
    #[ts(type = "[number, number, number]")]
    pub linear_velocity: DVec3,
    #[ts(type = "[number, number, number]")]
    pub angular_velocity: DVec3,
}

impl SimplePhysicsComponent {
    pub fn new(linear_velocity: DVec3, angular_velocity: DVec3) -> Self {
        Self {
            id: acquire_next_id(),
            linear_velocity,
            angular_velocity,
            is_static: false,
        }
    }

    pub fn from_mass() -> Self {
        Self {
            id: acquire_next_id(),
            linear_velocity: DVec3::new(0.0, 0.0, 0.0),
            angular_velocity: DVec3::new(0.0, 0.0, 0.0),
            is_static: false,
        }
    }

    pub fn new_static() -> Self {
        Self {
            id: acquire_next_id(),
            linear_velocity: DVec3::new(0.0, 0.0, 0.0),
            angular_velocity: DVec3::new(0.0, 0.0, 0.0),
            is_static: true,
        }
    }

    pub fn new_static_with_transform(linear_velocity: DVec3, angular_velocity: DVec3) -> Self {
        Self {
            id: acquire_next_id(),
            linear_velocity,
            angular_velocity,
            is_static: true,
        }
    }
}
