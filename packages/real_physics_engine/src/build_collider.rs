use rapier3d_f64::prelude::ColliderBuilder;
use serde::{Deserialize, Serialize};

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

pub fn build_collider(shape_description: &ShapeDescription) -> ColliderBuilder {
    let collider_builder = match shape_description {
        ShapeDescription::Ball(ball_description) => ColliderBuilder::ball(ball_description.radius),
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
