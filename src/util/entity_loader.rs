use crate::ecs::entity::Entity;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::ecs_components::physics::real_physics_component::{
    RealPhysicsComponent, ShapeDescription,
};
use crate::ecs_components::physics::simple_physics_component::SimplePhysicsComponent;
use crate::ecs_components::rendering::mesh_component::{MeshComponent, MeshDescription};
use crate::math::sin_cos::f64_to_dbig;
use crate::util::strip_json_line_comments::strip_json_line_comments;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicsDescription {
    pub mass: f64,
    pub shape: Option<ShapeDescription>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityDescription {
    pub mesh: Option<MeshDescription>,
    pub physics: Option<PhysicsDescription>,
}

pub fn load_entity(path: &str) -> Entity {
    let input_json = fs::read_to_string(path)
        .expect(format!("Failed to to read a entity description file {}", path).as_str());
    let description: EntityDescription =
        serde_json::from_str(&strip_json_line_comments(&input_json)).unwrap();

    let mut entity = Entity::new(Some(path));

    match description.mesh {
        None => (),
        Some(mesh_description) => entity
            .add_component(MeshComponent::from_description(mesh_description))
            .unwrap(),
    }

    match description.physics {
        None => (),
        Some(physics_description) => {
            entity
                .add_component(SimplePhysicsComponent::from_mass(f64_to_dbig(
                    physics_description.mass,
                )))
                .unwrap();
            match physics_description.shape {
                None => (),
                Some(shape_description) => entity
                    .add_component(RealPhysicsComponent::new(shape_description))
                    .unwrap(),
            }
        }
    }

    entity.add_component(TransformComponent::new()).unwrap();

    entity
}
