use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::ComponentsTypes;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use rapier3d_f64::prelude::ColliderBuilder;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealPhysicsComponent {
    pub id: u64,
    pub shape_description: ShapeDescription,
}

impl_component!(RealPhysicsComponent, false);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BallColliderDescription {
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxColliderDescription {
    pub size_x: f64,
    pub size_y: f64,
    pub size_z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CylinderColliderDescription {
    pub height: f64,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConeColliderDescription {
    pub height: f64,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShapeDescription {
    Ball(BallColliderDescription),
    Box(BoxColliderDescription),
    Cylinder(CylinderColliderDescription),
    Cone(ConeColliderDescription),
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

    pub fn build_collider(&self) -> ColliderBuilder {
        let collider_builder = match &self.shape_description {
            ShapeDescription::Ball(ball_description) => {
                ColliderBuilder::ball(ball_description.radius)
            }
            ShapeDescription::Box(box_description) => ColliderBuilder::cuboid(
                box_description.size_x * 0.5,
                box_description.size_y * 0.5,
                box_description.size_z * 0.5,
            ),
            ShapeDescription::Cylinder(cylinder_description) => ColliderBuilder::cylinder(
                cylinder_description.height * 0.5,
                cylinder_description.radius,
            ),
            ShapeDescription::Cone(cone_description) => {
                ColliderBuilder::cone(cone_description.height * 0.5, cone_description.radius)
            }
        };

        collider_builder
    }
}
