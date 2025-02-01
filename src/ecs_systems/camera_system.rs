use crate::core::game_state::GameState;
use crate::ecs::component_trait::ComponentTypes;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
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
        let entity = ecs.find_first_by_components(&[
            &ComponentTypes::CameraFocusComponent,
            &ComponentTypes::TransformComponent,
        ]);
        match entity {
            Some(entity) => {
                let transform = entity.components.transform.as_ref().unwrap();

                if entity
                    .components
                    .has(&ComponentTypes::FirstPersonCameraControlComponent)
                {
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
            None => {
                println!(
                    "Cannot update camera as there is no entity with CameraFocus and Transform"
                );
            }
        }
    }
}
