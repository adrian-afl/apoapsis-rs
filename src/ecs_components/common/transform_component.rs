use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::ecs::entity::ComponentTypes;
use crate::impl_component;
use crate::math::decimal_vector_3d::DecimalVector3d;
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransformComponent {
    pub id: u64,
    pub position: DecimalVector3d,
    pub orientation: DQuat,
    pub scale: DVec3,
}

impl_component!(TransformComponent, false);

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            id: acquire_next_id(),
            position: DecimalVector3d::zero(),
            orientation: DQuat::IDENTITY.clone(),
            scale: DVec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn from_position(position: DecimalVector3d) -> TransformComponent {
        TransformComponent {
            id: acquire_next_id(),
            position,
            orientation: DQuat::IDENTITY.clone(),
            scale: DVec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn from_position_orientation(
        position: DecimalVector3d,
        orientation: DQuat,
    ) -> TransformComponent {
        TransformComponent {
            id: acquire_next_id(),
            position,
            orientation,
            scale: DVec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn from_position_orientation_scale(
        position: DecimalVector3d,
        orientation: DQuat,
        scale: DVec3,
    ) -> TransformComponent {
        TransformComponent {
            id: acquire_next_id(),
            position,
            orientation,
            scale,
        }
    }
}
