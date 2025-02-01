use crate::ecs::component_trait::ComponentTypes;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use glam::DVec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetPhysicsKinematicsComponent {
    pub id: u64,
    // position and orientation can be set by SetBodyTransformComponent (if i already implemented it)
    pub linear_velocity: Option<DVec3>,
    pub angular_velocity: Option<DVec3>,
}

impl_component!(SetPhysicsKinematicsComponent);

impl SetPhysicsKinematicsComponent {
    pub fn new(linear_velocity: Option<DVec3>, angular_velocity: Option<DVec3>) -> Self {
        Self {
            id: acquire_next_id(),
            linear_velocity,
            angular_velocity,
        }
    }
}
