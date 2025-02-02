use crate::component_trait::acquire_next_id;
use real_physics_engine::build_collider::ShapeDescription;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealPhysicsComponent {
    pub id: u64,
    pub shape_description: ShapeDescription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealPhysicsDescription {
    pub shape: ShapeDescription,
    // pub dynamic: false, // there will be almost no use for this, uncomment when needed
}

impl RealPhysicsComponent {
    pub fn new(shape_description: ShapeDescription) -> Self {
        Self {
            id: acquire_next_id(),
            shape_description,
        }
    }
}
