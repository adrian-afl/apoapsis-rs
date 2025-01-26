use crate::body::body_definitions::load_body_data;
use crate::celestial_rendering::renderer::Renderer;
use crate::component_types;
use crate::config::Config;
use crate::core::game_event_system::{GameEvent, GameEventSystem};
use crate::core::game_state::GameState;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::entity::Entity;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_components::camera::camera_focus_component::CameraFocusComponent;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::ecs_systems::rendering_system::RenderingSystem;
use crate::ecs_systems::universe_simulation_updater_system::UniverseSimulationUpdaterSystem;
use crate::input::controls::{ControlEvent, ControlMapItem, Controls};
use crate::input::keyboard_input::KeyboardInput;
use crate::input::mouse_input::MouseInput;
use crate::math::decimal_vector_3d::DecimalVector3d;
use crate::simulation::simulation::Simulation;
use glam::DQuat;
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
        let ecs_systems: Vec<Box<dyn SystemTrait>> = vec![
            Box::new(UniverseSimulationUpdaterSystem::new(
                universe_simulation.clone(),
            )),
            Box::new(RenderingSystem::new(
                toolkit.clone(),
                renderer.clone(),
                universe_simulation.clone(),
            )),
        ];

        let mouse_input = MouseInput::new(window.clone(), controls.clone());
        let keyboard_input = KeyboardInput::new(window.clone(), controls.clone());

        {
            let mut universe = universe_simulation.lock().unwrap();
            let mut renderer = renderer.lock().unwrap();

            renderer
                .add_hierarchy_to_universe_simulation(
                    &mut universe,
                    &load_body_data("media/universe/solar_system/sun/sun.json"),
                )
                .expect("Failed to load sun.json");
        }

        {
            let mut player_entity = Entity::new(Some("Player"));
            let mut universe = universe_simulation.lock().unwrap();
            let earth_position = &universe.get_body("earth").position;
            let player_initial_position =
                earth_position + DecimalVector3d::from_f64(0.0, 0.0, 9398000.0);
            state
                .lock()
                .unwrap()
                .current_camera
                .position
                .assign(&player_initial_position);
            player_entity
                .add_component(TransformComponent::from_position(player_initial_position))
                .unwrap();
            player_entity
                .add_component(CameraFocusComponent::new())
                .unwrap();
            ecs.lock().unwrap().add(player_entity).unwrap();
        }

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
            system.update(self.state.clone(), self.ecs.clone());
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

        {
            let mut ecs = self.ecs.lock().unwrap();
            let mut focus = ecs
                .find_first_by_components_mut(component_types!(
                    CameraFocusComponent,
                    TransformComponent
                ))
                .unwrap();
            let mut transform = focus
                .get_first_component_mut::<TransformComponent>()
                .unwrap();

            let mut universe = self.universe_simulation.lock().unwrap();
            let earth_position = &universe.get_body("earth").position;
            println!("earth_position {earth_position}");
            let player_position = earth_position + DecimalVector3d::from_f64(0.0, 0.0, 9398000.0);
            transform.position.assign(&player_position);
        }

        self.game_event_system.cleanup();
    }
}
