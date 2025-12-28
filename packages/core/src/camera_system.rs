use ecs::component_trait::Components;
use ecs::ecs_world::ECSWorld;
use renderer_common::camera::Camera;
use std::f64::consts::PI;

pub struct CameraSystem {}

impl CameraSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, camera: &mut Camera, ecs: &mut ECSWorld) {
        // println!("CameraSystem / update");

        let entity =
            ecs.find_first_by_components(&[&Components::CameraFocus, &Components::Transform]);
        match entity {
            Some(entity) => {
                let transform = entity.components.transform.as_ref().unwrap();

                if entity.components.has(&Components::FirstPersonCameraControl) {
                    let first_person_component = entity
                        .components
                        .first_person_camera_control
                        .as_ref()
                        .unwrap();
                    camera.position.assign(&transform.position);
                    camera.orientation = transform.orientation.inverse();
                    camera.set_perspective(
                        first_person_component.fov * (PI / 180.0),
                        4.0 / 3.0,
                        0.1,
                        9999999999.0,
                    );

                    camera.update();

                    // println!("Cam pos is now {}", camera.position);
                }
            }
            None => {
                // println!(
                //     "Cannot update camera as there is no entity with CameraFocus and Transform"
                // );
            }
        }
    }
}
