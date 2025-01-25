use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::ecs_components::rendering::mesh_component::MaterialDescription;
use crate::impl_component;
use crate::math::decimal_vector_3d::DecimalVector3d;
use dashu_float::DBig;
use serde::Deserialize;
use std::any::{Any, TypeId};

#[derive(Clone, Debug)]
pub struct SimplePhysicsComponent {
    pub id: u64,
    pub mass: DBig,
    pub linear_velocity: DecimalVector3d,
    pub angular_velocity: DecimalVector3d,
}

impl_component!(SimplePhysicsComponent, false);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplePhysicsDescription {
    pub mass: f64,
}

impl SimplePhysicsComponent {
    pub fn new(
        mass: DBig,
        linear_velocity: DecimalVector3d,
        angular_velocity: DecimalVector3d,
    ) -> Self {
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
            linear_velocity: DecimalVector3d::zero(),
            angular_velocity: DecimalVector3d::zero(),
        }
    }
}
