use crate::component_trait::acquire_next_id;
use glam::DVec3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct SetPhysicsKinematicsComponent {
    pub id: u64,
    // position and orientation can be set by SetBodyTransformComponent (if i already implemented it)
    #[ts(type = "[number, number, number] | null")]
    pub linear_velocity: Option<DVec3>,
    #[ts(type = "[number, number, number] | null")]
    pub angular_velocity: Option<DVec3>,
}

impl SetPhysicsKinematicsComponent {
    pub fn new(linear_velocity: Option<DVec3>, angular_velocity: Option<DVec3>) -> Self {
        Self {
            id: acquire_next_id(),
            linear_velocity,
            angular_velocity,
        }
    }
}
