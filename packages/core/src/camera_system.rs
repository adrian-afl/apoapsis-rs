use ecs::component_trait::Components;
use ecs::ecs_world::ECSWorld;
use renderer_common::camera::Camera;

pub struct CameraSystem {}

impl CameraSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, camera: &mut Camera, ecs: &mut ECSWorld) {
        println!("CameraSystem / update");

        let entity =
            ecs.find_first_by_components(&[&Components::CameraFocus, &Components::Transform]);
        match entity {
            Some(entity) => {
                let transform = entity.components.transform.as_ref().unwrap();

                if entity.components.has(&Components::FirstPersonCameraControl) {
                    camera.position.assign(&transform.position);
                    camera.orientation = transform.orientation.clone();

                    camera.update();

                    println!("Cam pos is now {}", camera.position);
                }
            }
            None => {
                println!(
                    "Cannot update camera as there is no entity with CameraFocus and Transform"
                );
            }
        }
    }
}
