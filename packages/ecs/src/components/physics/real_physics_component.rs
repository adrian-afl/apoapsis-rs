use crate::component_trait::acquire_next_id;
use glam::DVec3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct BallColliderDescription {
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct BoxColliderDescription {
    pub size_x: f64,
    pub size_y: f64,
    pub size_z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct CylinderColliderDescription {
    pub height: f64,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ConeColliderDescription {
    pub height: f64,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct TriMeshColliderDescription {
    #[ts(type = "[number, number, number][]")]
    pub vertices: Vec<DVec3>,
    pub indices: Vec<[u32; 3]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum ShapeDescription {
    Ball(BallColliderDescription),
    Box(BoxColliderDescription),
    Cylinder(CylinderColliderDescription),
    Cone(ConeColliderDescription),
    TriMesh(TriMeshColliderDescription),
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct RealPhysicsComponent {
    pub id: u64,
    pub shape_description: ShapeDescription,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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
