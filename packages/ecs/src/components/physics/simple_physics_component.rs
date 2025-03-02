use crate::component_trait::acquire_next_id;
use dashu_float::DBig;
use glam::DVec3;
use math::decimal_vector_3d::DecimalVector3d;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplePhysicsComponent {
    pub id: u64,
    pub mass: DBig,
    pub linear_velocity: DVec3,
    pub angular_velocity: DVec3,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplePhysicsDescription {
    pub mass: f64,
}

impl SimplePhysicsComponent {
    pub fn new(mass: DBig, linear_velocity: DVec3, angular_velocity: DVec3) -> Self {
        Self {
            id: acquire_next_id(),
            mass,
            linear_velocity,
            angular_velocity,
        }
    }

    pub fn from_mass(mass: DBig) -> Self {
        Self {
            id: acquire_next_id(),
            mass,
            linear_velocity: DVec3::new(0.0, 0.0, 0.0),
            angular_velocity: DVec3::new(0.0, 0.0, 0.0),
        }
    }
}
