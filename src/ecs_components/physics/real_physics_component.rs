use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use rapier3d_f64::prelude::{ColliderBuilder, RigidBodyBuilder};
use std::any::{Any, TypeId};

#[derive(Clone, Debug)]
pub struct RealPhysicsComponent {
    pub id: u64,
    pub collider_builder: ColliderBuilder,
    pub rigid_body_builder: RigidBodyBuilder,
}

impl_component!(RealPhysicsComponent, false);

impl RealPhysicsComponent {
    pub fn new(collider_builder: ColliderBuilder, rigid_body_builder: RigidBodyBuilder) -> Self {
        Self {
            id: acquire_next_id(),
            collider_builder,
            rigid_body_builder,
        }
    }
}
