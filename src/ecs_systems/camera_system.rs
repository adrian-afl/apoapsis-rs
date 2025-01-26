use crate::component_types;
use crate::core::game_state::GameState;
use crate::ecs::component_trait::component_type;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_components::camera::camera_focus_component::CameraFocusComponent;
use crate::ecs_components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use crate::ecs_components::common::transform_component::TransformComponent;
use std::sync::{Arc, Mutex};

pub struct CameraSystem {}

impl CameraSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl SystemTrait for CameraSystem {
    fn update(&mut self, game_state: Arc<Mutex<GameState>>, ecs: Arc<Mutex<ECSWorld>>) {
        let ecs = ecs.lock().unwrap();
        let entity = ecs
            .find_first_by_components(component_types!(CameraFocusComponent, TransformComponent));
        match entity {
            Ok(entity) => {
                let transform = entity.get_first_component::<TransformComponent>().unwrap();

                if entity.has_component_of_type::<FirstPersonCameraControlComponent>() {
                    let mut locked_state = game_state.lock().unwrap();
                    locked_state
                        .current_camera
                        .position
                        .assign(&transform.position);
                    locked_state.current_camera.orientation = transform.orientation.clone();

                    locked_state.current_camera.update();

                    println!("Cam pos is now {}", locked_state.current_camera.position);
                }
            }
            Err(_) => {
                println!(
                    "Cannot update camera as there is no entity with CameraFocus and Transform"
                );
            }
        }
    }
}
