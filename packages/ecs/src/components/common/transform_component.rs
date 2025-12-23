use crate::component_trait::acquire_next_id;
use glam::{DQuat, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct TransformComponent {
    pub id: u64,
    pub position: DecimalVector3d,
    #[ts(type = "[number, number, number, number]")]
    pub orientation: DQuat,
    #[ts(type = "[number, number, number]")]
    pub scale: DVec3,
}

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
