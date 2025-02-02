use crate::camera_system::CameraSystem;
use celestial_renderer::renderer::Renderer;
use celestial_renderer::rendering_system::RenderingSystem;
use ecs::component_trait::Components;
use ecs::components::camera::camera_focus_component::CameraFocusComponent;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::ecs_world::{ECSWorld, ECSWorldSerializedRepresentation};
use ecs::entity::Entity;
use ecs::game_state::GameState;
use ecs::system_trait::SystemTrait;
use input::control_event_system::{GameEvent, GameEventSystem};
use input::controls::{ControlMapItem, Controls};
use input::keyboard_input::KeyboardInput;
use input::mouse_input::MouseInput;
use math::decimal_vector_3d::DecimalVector3d;
use real_physics_engine::physics_system::PhysicsSystem;
use renderer_common::resolution_config::ResolutionConfig;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, RwLock};
use ui_renderer::ui_system::UIRenderer;
use universe_simulation::body_definitions::load_body_data;
use universe_simulation::simulation::Simulation;
use universe_simulation::universe_simulation_updater_system::UniverseSimulationUpdaterSystem;
use vengine_rs::core::toolkit::VEToolkit;
use winit::window::Window;

pub struct Game {
    toolkit: Arc<VEToolkit>,
    window: Arc<Mutex<Window>>,

    pub config: ResolutionConfig,

    universe_simulation: Arc<RwLock<Simulation>>,
    renderer: Arc<Mutex<Renderer>>,
    state: Arc<Mutex<GameState>>,

    ecs: Arc<Mutex<ECSWorld>>,
    ecs_systems: Vec<Box<dyn SystemTrait>>,

    pub mouse_input: MouseInput,
    pub keyboard_input: KeyboardInput,
    controls: Arc<Controls>,
    game_event_system: Arc<GameEventSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSerializedRepresentation {
    pub ecs: ECSWorldSerializedRepresentation,
    // pub state: GameState,
}

impl Game {
    pub fn new(toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>) -> Self {
        let config = ResolutionConfig {
            width: 640,
            height: 480,
        };

        let game_event_system = Arc::new(GameEventSystem::new());
        let controls = Arc::new(Controls::new(game_event_system.clone()));

        let universe_simulation = Arc::new(RwLock::new(Simulation::new()));
        let ui_renderer = UIRenderer::new(toolkit.clone(), &config);
        let renderer = Arc::new(Mutex::from(Renderer::new(
            toolkit.clone(),
            &config,
            ui_renderer.ui_drawer.clone(),
        )));
        let state = Arc::new(Mutex::from(GameState::new()));
        let ecs = Arc::new(Mutex::from(ECSWorld::new()));
        let ecs_systems: Vec<Box<dyn SystemTrait>> = vec![
            Box::new(UniverseSimulationUpdaterSystem::new(
                universe_simulation.clone(),
            )),
            Box::new(CameraSystem::new()),
            Box::new(PhysicsSystem::new(universe_simulation.clone())),
            Box::new(RenderingSystem::new(
                toolkit.clone(),
                renderer.clone(),
                universe_simulation.clone(),
            )),
        ];

        let mouse_input = MouseInput::new(window.clone(), controls.clone());
        let keyboard_input = KeyboardInput::new(window.clone(), controls.clone());

        {
            let mut universe = universe_simulation.try_write().unwrap();
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
            let universe = universe_simulation.try_read().unwrap();
            let earth_position = &universe.get_body("earth").position;
            let player_initial_position =
                earth_position + DecimalVector3d::from_f64(0.0, 0.0, 9398000.0);
            state
                .lock()
                .unwrap()
                .current_camera
                .position
                .assign(&player_initial_position);
            player_entity.components.transform =
                Some(TransformComponent::from_position(player_initial_position));
            player_entity.components.camera_focus = Some(CameraFocusComponent::new());
            player_entity.components.first_person_camera_control =
                Some(FirstPersonCameraControlComponent::new());
            ecs.lock().unwrap().add(player_entity);
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
            let focus = ecs
                .find_first_by_components_mut(&[&Components::CameraFocus, &Components::Transform])
                .unwrap();
            let transform = focus.components.transform.as_mut().unwrap();

            let universe = self.universe_simulation.try_read().unwrap();
            let earth_position = &universe.get_body("earth").position;
            println!("earth_position {earth_position}");
            let player_position = earth_position + DecimalVector3d::from_f64(0.0, 0.0, 9398000.0);
            transform.position.assign(&player_position);
            println!("transform.position {}", transform.position);
            println!(
                "transform distance to earth {}",
                transform.position.distance_to(&earth_position)
            );
        }

        self.game_event_system.cleanup();

        let res = self.serialize();
        self.deserialize_in_place(&res);
    }

    pub fn serialize(&self) -> String {
        let repr = GameSerializedRepresentation {
            ecs: self.ecs.lock().unwrap().serialize(),
            // state: self.state.lock().unwrap().clone(),
        };
        serde_json::to_string(&repr).unwrap()
    }

    pub fn deserialize_in_place(&mut self, str: &str) {
        let repr: GameSerializedRepresentation = serde_json::from_str(str).unwrap();
        // self.state = Arc::new(Mutex::from(repr.state));
        self.ecs = Arc::new(Mutex::from(ECSWorld::deserialize(repr.ecs)));
    }
}
