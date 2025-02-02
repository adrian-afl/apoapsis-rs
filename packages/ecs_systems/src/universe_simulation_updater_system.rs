use crate::game_state::GameState;
use crate::system_trait::SystemTrait;
use celestial_renderer::simulation::simulation::Simulation;
use ecs::ecs_world::ECSWorld;
use math::sin_cos::f64_to_dbig;
use std::sync::{Arc, Mutex, RwLock};

pub struct UniverseSimulationUpdaterSystem {
    universe: Arc<RwLock<Simulation>>,
}

impl UniverseSimulationUpdaterSystem {
    pub fn new(universe: Arc<RwLock<Simulation>>) -> Self {
        Self { universe }
    }
}

impl SystemTrait for UniverseSimulationUpdaterSystem {
    fn update(&mut self, game_state: Arc<Mutex<GameState>>, ecs: Arc<Mutex<ECSWorld>>) {
        println!("UniverseSimulationUpdaterSystem / update");

        let game_state = game_state.lock().unwrap();
        let game_time = &f64_to_dbig(1230.0); //&game_state.lock().unwrap().current_game_time;

        println!(
            "game_time camera position {}",
            &game_state.current_camera.position
        );
        self.universe
            .try_write()
            .unwrap()
            .update(&game_state.current_camera.position, game_time);
    }
}
