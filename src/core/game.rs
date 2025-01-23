use crate::celestial_rendering::renderer::Renderer;
use crate::config::Config;
use crate::core::game_state::GameState;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_systems::universe_simulation_updater_system::UniverseSimulationUpdaterSystem;
use crate::simulation::simulation::Simulation;
use std::sync::{Arc, Mutex};
use vengine_rs::core::toolkit::VEToolkit;
use winit::window::Window;

pub struct Game {
    toolkit: Arc<VEToolkit>,

    pub config: Config,

    universe_simulation: Arc<Mutex<Simulation>>,
    renderer: Arc<Mutex<Renderer>>,
    state: Arc<Mutex<GameState>>,

    ecs: Arc<Mutex<ECSWorld>>,
    ecs_systems: Vec<Box<dyn SystemTrait>>,
}

impl Game {
    pub fn new(toolkit: Arc<VEToolkit>) -> Self {
        let config = Config::new(640, 480);

        let universe_simulation = Arc::new(Mutex::from(Simulation::new(toolkit.clone())));
        let renderer = Arc::new(Mutex::from(Renderer::new(toolkit.clone(), &config)));
        let state = Arc::new(Mutex::from(GameState::new()));
        let ecs = Arc::new(Mutex::from(ECSWorld::new()));
        let ecs_systems: Vec<Box<dyn SystemTrait>> = vec![Box::new(
            UniverseSimulationUpdaterSystem::new(universe_simulation.clone()),
        )];

        Self {
            toolkit: toolkit.clone(),

            config,

            universe_simulation,
            renderer,
            state,
            ecs,
            ecs_systems,
        }
    }

    pub fn update(&mut self, window: &mut Window) {
        {
            let mut state = self.state.lock().unwrap();
            state.update_time()
        }
        for system in &mut self.ecs_systems {
            system.update(self.state.clone(), self.ecs.clone());
        }
    }
}
