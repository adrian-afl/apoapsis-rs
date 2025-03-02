use core::game_context::GameContext;
use dashu_float::DBig;
use ecs::component_trait::Components;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::physics::simple_physics_component::SimplePhysicsComponent;
use ecs::components::rendering::mesh_component::{
    MaterialDescription, MeshComponent, MeshDescription,
};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::dvec3;
use input::controls_mapping::ControlMapItem;
use math::decimal_vector_3d::DecimalVector3d;
use math::get_quat_directions::get_quat_directions;

pub struct DebugBoxSpawnerPlugin {}

impl DebugBoxSpawnerPlugin {
    pub fn new(_: &GameContext, _: &mut ECSWorld) -> Self {
        Self {}
    }

    pub fn update(&self, context: &GameContext, ecs: &mut ECSWorld) {
        if context
            .controls
            .was_control_activated(ControlMapItem::FlightShoot)
        {
            println!("PRESSED");
            let camera_entity = ecs.find_first_by_components(&[
                &Components::CameraFocus,
                &Components::Transform,
                &Components::SimplePhysics,
            ]);
            if let Some(camera_entity) = camera_entity {
                let camera_transform = camera_entity.components.transform.as_ref().unwrap();
                let camera_simple_physics =
                    camera_entity.components.simple_physics.as_ref().unwrap();

                let mut box_entity = Entity::noname();
                let forward_vector = get_quat_directions(camera_transform.orientation).forwards;
                box_entity.components.transform =
                    Some(TransformComponent::from_position_orientation(
                        &camera_transform.position
                            + DecimalVector3d::from_dvec3(&forward_vector * 2.0),
                        camera_transform.orientation,
                    ));
                box_entity.components.simple_physics = Some(SimplePhysicsComponent::new(
                    DBig::ONE,
                    camera_simple_physics.linear_velocity + forward_vector,
                    camera_simple_physics.angular_velocity,
                ));
                // box_entity.components.real_physics = Some(RealPhysicsComponent::new(
                //     ShapeDescription::Box(BoxColliderDescription {
                //         size_x: 1.0,
                //         size_y: 1.0,
                //         size_z: 1.0,
                //     }),
                // ));
                box_entity
                    .components
                    .mesh
                    .push(MeshComponent::from_description(MeshDescription {
                        geometry_path: "media/smoothbox.pnut.raw".to_owned(),
                        material: MaterialDescription::default().color_solid(dvec3(1.0, 0.5, 0.5)),
                    }));

                ecs.add(box_entity);
                println!("SPAWNED");
            }
        }
    }
}
