use crate::celestial_rendering::renderer::Renderer;
use crate::config::Config;
use crate::core::game_event_system::{GameEvent, GameEventSystem};
use crate::core::game_state::GameState;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_systems::universe_simulation_updater_system::UniverseSimulationUpdaterSystem;
use crate::input::controls::{ControlEvent, ControlMapItem, Controls};
use crate::input::keyboard_input::KeyboardInput;
use crate::input::mouse_input::MouseInput;
use crate::simulation::simulation::Simulation;
use std::sync::{Arc, LockResult, Mutex};
use vengine_rs::core::toolkit::VEToolkit;
use winit::window::Window;

pub struct Game {
    toolkit: Arc<VEToolkit>,
    window: Arc<Mutex<Window>>,

    pub config: Config,

    universe_simulation: Arc<Mutex<Simulation>>,
    renderer: Arc<Mutex<Renderer>>,
    state: Arc<Mutex<GameState>>,

    ecs: Arc<Mutex<ECSWorld>>,
    ecs_systems: Vec<Box<dyn SystemTrait>>,

    pub mouse_input: MouseInput,
    pub keyboard_input: KeyboardInput,
    controls: Arc<Controls>,
    game_event_system: Arc<GameEventSystem>,
}

impl Game {
    pub fn new(toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>) -> Self {
        let config = Config::new(640, 480);

        let game_event_system = Arc::new(GameEventSystem::new());
        let controls = Arc::new(Controls::new(game_event_system.clone()));

        let universe_simulation = Arc::new(Mutex::from(Simulation::new(toolkit.clone())));
        let renderer = Arc::new(Mutex::from(Renderer::new(toolkit.clone(), &config)));
        let state = Arc::new(Mutex::from(GameState::new()));
        let ecs = Arc::new(Mutex::from(ECSWorld::new()));
        let ecs_systems: Vec<Box<dyn SystemTrait>> = vec![Box::new(
            UniverseSimulationUpdaterSystem::new(universe_simulation.clone()),
        )];

        let mouse_input = MouseInput::new(window.clone(), controls.clone());
        let keyboard_input = KeyboardInput::new(window.clone(), controls.clone());

        Self {
            toolkit: toolkit.clone(),
            window: window.clone(),

            config,

            universe_simulation,
            renderer,
            state,
            ecs,
            ecs_systems,

            mouse_input,
            keyboard_input,
            controls,
            game_event_system,
        }
    }

    pub fn update(&mut self) {
        {
            let mut state = self.state.lock().unwrap();
            state.update_time()
        }
        for system in &mut self.ecs_systems {
            //    system.update(self.state.clone(), self.ecs.clone());
        }

        let events = self.game_event_system.get_events::<Game>();
        for event in events {
            println!("{:?}", event);
            if let GameEvent::ControlActivate(control) = event {
                if control == ControlMapItem::Pause {
                    match self.mouse_input.is_cursor_locked() {
                        true => self.mouse_input.unlock_cursor(),
                        false => self.mouse_input.lock_cursor(),
                    }
                }
            }
        }

        self.game_event_system.cleanup();
    }
}
