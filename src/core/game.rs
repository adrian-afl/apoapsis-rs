use crate::app::CelestialRendererApp;
use crate::celestial_rendering::scene::camera::Camera;
use crate::config::Config;
use crate::core::game_state::GameState;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::math::sin_cos::f64_to_dbig;
use crate::simulation::simulation::Simulation;
use std::ops::Add;
use std::sync::{Arc, Mutex};

pub struct Game {
    start_time: f64,
    last_time: f64,

    pub config: Config,
    //
    // camera: Camera,
    universe_simulation: Arc<Mutex<Simulation>>,
    renderer: Arc<Mutex<CelestialRendererApp>>,
    state: Arc<Mutex<GameState>>,

    ecs: Arc<Mutex<ECSWorld>>,
    ecs_systems: Vec<Box<dyn SystemTrait>>,
}

impl Game {
    pub fn new() -> Self {
        // TODO this is going to be a big one
        Self {}
    }

    pub fn update(&mut self, delta_time: f64) {
        {
            let mut state = self.state.lock().unwrap();
            state.current_time = (&state.current_time).add(f64_to_dbig(delta_time));
        }
        for system in &mut self.ecs_systems {
            system.update(self.state.clone(), self.ecs.clone(), delta_time);
        }
    }
}
