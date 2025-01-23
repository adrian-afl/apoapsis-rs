use crate::component_types;
use crate::core::game_state::GameState;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_components::camera::camera_focus_component::CameraFocusComponent;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::simulation::simulation::Simulation;
use std::sync::{Arc, Mutex};

pub struct UniverseSimulationUpdaterSystem {
    universe: Arc<Mutex<Simulation>>,
}

impl UniverseSimulationUpdaterSystem {
    pub fn new(universe: Arc<Mutex<Simulation>>) -> Self {
        Self { universe }
    }
}

impl SystemTrait for UniverseSimulationUpdaterSystem {
    fn update(&mut self, game_state: Arc<Mutex<GameState>>, ecs: Arc<Mutex<ECSWorld>>) {
        let position = {
            let ecs = ecs.lock().unwrap();
            let camera_focus = ecs.find_first_by_components(component_types!(
                CameraFocusComponent,
                TransformComponent
            ));
            match camera_focus {
                Ok(camera_focus) => {
                    let transform = camera_focus
                        .get_first_component::<TransformComponent>()
                        .unwrap();
                    transform.position.clone()
                }
                Err(_) => {
                    println!("Cannot update universe simulation as there is no entity with CameraFocus and Transform");
                    return;
                }
            }
        };
        let game_time = &game_state.lock().unwrap().current_game_time;
        self.universe.lock().unwrap().update(&position, game_time);
    }
}
