use crate::camera_system::CameraSystem;
use crate::game_stage::{GameStage, GameUpdateData, StageTransition};
use crate::stages::stages_stack::StageStack;
use crate::time_counter::TimeCounter;
use celestial_renderer::renderer::Renderer;
use celestial_renderer::rendering_system::RenderingSystem;
use ecs::components::ui::cursor_type::UICursorType;
use glam::DVec2;
use input::controls::Controls;
use real_physics_engine::physics_system::PhysicsSystem;
use renderer_common::camera::Camera;
use renderer_common::resolution_config::ResolutionConfig;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use ui_renderer::ui_cursor_system::UICursorSystem;
use ui_renderer::ui_raycast_system::{UIRaycastResultItem, UIRaycastSystem};
use ui_renderer::ui_system::UISystem;
use universe_simulation::body_definitions::load_body_data;
use universe_simulation::simulation::Simulation;
use universe_simulation::universe_simulation_updater_system::UniverseSimulationUpdaterSystem;
use vengine_rs::core::toolkit::VEToolkit;
use winit::window::{CursorIcon, Window};

pub struct Game {
    toolkit: Arc<VEToolkit>,
    window: Arc<Mutex<Window>>,

    pub config: ResolutionConfig,

    universe_simulation: Simulation,
    time_counter: TimeCounter,

    pub controls: Controls,

    stage_stack: StageStack,

    // systems, owned always running
    current_camera: Camera,

    universe_simulation_updater_system: UniverseSimulationUpdaterSystem,
    camera_system: CameraSystem,
    ui_system: UISystem,
    rendering_system: RenderingSystem,
    physics_system: PhysicsSystem,
    ui_cursor_system: UICursorSystem,
    ui_raycast_system: UIRaycastSystem,
    ui_raycast_result: Vec<UIRaycastResultItem>,
}

impl Game {
    pub fn new(
        toolkit: Arc<VEToolkit>,
        window: Arc<Mutex<Window>>,
        initial_stage: Box<dyn GameStage>,
    ) -> Self {
        let config = ResolutionConfig {
            width: 640,
            height: 480,
        };

        let controls = Controls::new(window.clone());

        let mut universe_simulation = Simulation::new();
        let ui_system = UISystem::new(toolkit.clone(), &config);
        let ui_cursor_system = UICursorSystem::new();
        let ui_raycast_system = UIRaycastSystem::new();
        let mut renderer = Renderer::new(toolkit.clone(), &config, ui_system.ui_drawer.clone());

        renderer
            .add_hierarchy_to_universe_simulation(
                &mut universe_simulation,
                &load_body_data("media/universe/solar_system/sun/sun.json"),
            )
            .expect("Failed to load sun.json");

        let universe_simulation_updater_system = UniverseSimulationUpdaterSystem::new();

        let camera_system = CameraSystem::new();
        let physics_system = PhysicsSystem::new();
        let rendering_system = RenderingSystem::new(toolkit.clone(), renderer);

        let time_counter = TimeCounter::new();

        let mut stage_stack = StageStack::new();
        // entrypoint to the game is here, this kicks off the whole thing:
        stage_stack.push(initial_stage);

        Self {
            toolkit: toolkit.clone(),
            window: window.clone(),

            config,

            universe_simulation,

            controls,

            time_counter,

            ui_system,
            camera_system,
            universe_simulation_updater_system,
            rendering_system,
            physics_system,
            ui_cursor_system,
            ui_raycast_system,
            ui_raycast_result: vec![],

            stage_stack,
            current_camera: Camera::new(),
        }
    }

    pub fn update(&mut self) {
        let window_size = self.window.lock().unwrap().inner_size();
        self.time_counter.update_time();

        let mut transition_from_update = StageTransition::DoNothing;

        if let Some(stage) = &self.stage_stack.head() {
            let mut stage = stage.lock().unwrap();
            transition_from_update = stage.update(GameUpdateData {
                total_time: self.time_counter.total_time,
                delta_time: self.time_counter.delta_time,
                universe: &self.universe_simulation,
                controls: &mut self.controls,
            });

            let stage_ecs = stage.get_ecs_world();

            self.camera_system
                .update(&mut self.current_camera, stage_ecs);

            self.universe_simulation_updater_system.update(
                &mut self.universe_simulation,
                stage_ecs,
                self.time_counter.delta_time,
            );

            self.physics_system.update(
                stage_ecs,
                &self.universe_simulation,
                self.time_counter.delta_time,
            );

            let normalized_cursor_pos = self.controls.mouse.get_cursor_absolute()
                / DVec2::new(window_size.width as f64, window_size.height as f64);

            self.ui_system.update(stage_ecs, normalized_cursor_pos);
            self.ui_raycast_system.update(
                &mut self.ui_raycast_result,
                stage_ecs,
                normalized_cursor_pos,
            );
            let cursor_system_result = self
                .ui_cursor_system
                .update(stage_ecs, &self.ui_raycast_result);
            match cursor_system_result.cursor_locked {
                true => {
                    if !self.controls.mouse.is_cursor_locked() {
                        self.controls.mouse.lock_cursor();
                    }
                }
                false => {
                    if self.controls.mouse.is_cursor_locked() {
                        self.controls.mouse.unlock_cursor();
                    }
                }
            }
            match cursor_system_result.cursor_type {
                UICursorType::Arrow => {
                    if self.controls.mouse.get_cursor_type() != CursorIcon::Default {
                        self.controls.mouse.set_cursor_type(CursorIcon::Default)
                    }
                }
                UICursorType::Pointer => {
                    if self.controls.mouse.get_cursor_type() != CursorIcon::Pointer {
                        self.controls.mouse.set_cursor_type(CursorIcon::Pointer)
                    }
                }
                UICursorType::Grab => {
                    if self.controls.mouse.get_cursor_type() != CursorIcon::Pointer {
                        self.controls.mouse.set_cursor_type(CursorIcon::Pointer)
                    }
                }
            }

            self.rendering_system.update(
                stage_ecs,
                &self.universe_simulation,
                &self.current_camera,
                self.time_counter.total_time,
                self.time_counter.delta_time,
            );
        }

        match transition_from_update {
            StageTransition::PushStage(stage) => self.stage_stack.push(stage),
            StageTransition::PopSelf => {
                self.stage_stack.pop();
            }
            StageTransition::DoNothing => (),
        }
    }
}
