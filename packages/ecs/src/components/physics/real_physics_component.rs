use crate::component_trait::acquire_next_id;
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SphereColliderDescription {
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct BoxColliderDescription {
    pub half_x: f64,
    pub half_y: f64,
    pub half_z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct TriMeshColliderDescription {
    pub cache_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum ColliderShape {
    Sphere(SphereColliderDescription),
    Box(BoxColliderDescription),
    TriMesh(TriMeshColliderDescription),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ColliderDescription {
    #[ts(type = "[number, number, number]")]
    pub offset: DVec3,
    #[ts(type = "[number, number, number, number]")]
    pub orientation: DQuat,
    pub mass: f64,
    pub shape: ColliderShape,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct RealPhysicsComponent {
    #[serde(skip, default = "acquire_next_id")]
    pub id: u64,
    pub collider_descriptions: Vec<ColliderDescription>,
    pub override_real_simulation_cutoff: Option<f64>,
}

impl RealPhysicsComponent {
    pub fn new(collider_descriptions: Vec<ColliderDescription>) -> Self {
        Self {
            id: acquire_next_id(),
            collider_descriptions,
            override_real_simulation_cutoff: None,
        }
    }

    pub fn with_override(
        collider_descriptions: Vec<ColliderDescription>,
        override_real_simulation_cutoff: f64,
    ) -> Self {
        Self {
            id: acquire_next_id(),
            collider_descriptions,
            override_real_simulation_cutoff: Some(override_real_simulation_cutoff),
        }
    }
}
