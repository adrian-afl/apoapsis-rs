use crate::camera_system::CameraSystem;
use crate::remote_api::remote_game_mode::{RemoteGameExecutionContext, RemoteGameMode};
use celestial_renderer::renderer::Renderer;
use celestial_renderer::rendering_system::RenderingSystem;
use common_util::profile;
use ecs::components::ui::cursor_type::UICursorType;
use ecs::components::ui::ui_text_component::UIFontSize;
use glam::DVec2;
use input::controls::{ControlEvent, Controls};
use input::controls_mapping::ControlMapItem;
use real_physics_engine::physics_system::PhysicsSystem;
use renderer_common::camera::Camera;
use renderer_common::resolution_config::ResolutionConfig;
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
    // toolkit: Arc<VEToolkit>,
    window: Option<Arc<Mutex<Window>>>,

    pub config: ResolutionConfig,

    universe_simulation: Simulation,

    pub controls: Option<Controls>,

    remote_game_mode: RemoteGameMode,

    // systems, owned always running
    current_camera: Camera,

    universe_simulation_updater_system: UniverseSimulationUpdaterSystem,
    camera_system: CameraSystem,
    ui_system: Option<UISystem>,
    rendering_system: Option<RenderingSystem>,
    physics_system: PhysicsSystem,
    ui_cursor_system: Option<UICursorSystem>,
    ui_raycast_system: Option<UIRaycastSystem>,
    ui_raycast_result: Vec<UIRaycastResultItem>,
}

impl Game {
    pub fn new(toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>) -> Self {
        let config = ResolutionConfig {
            width: 640,
            height: 480,
        };

        let controls = Controls::new(window.clone());

        let mut universe_simulation = Simulation::new();
        let ui_system = UISystem::new(toolkit.clone(), &config);
        let ui_cursor_system = UICursorSystem::new();
        let ui_raycast_system = UIRaycastSystem::new();
        let renderer = Renderer::new(toolkit.clone(), &config, ui_system.ui_drawer.clone());

        universe_simulation
            .add_hierarchy(
                &config,
                &load_body_data("media/universe/solar_system/sun/sun.json"),
                None,
            )
            .expect("Failed to load sun.json");

        let universe_simulation_updater_system = UniverseSimulationUpdaterSystem::new();

        let camera_system = CameraSystem::new();
        let physics_system = PhysicsSystem::new();
        let rendering_system = RenderingSystem::new(toolkit.clone(), renderer);

        let remote_game_mode = RemoteGameMode::new();

        Self {
            // toolkit: toolkit.clone(),
            window: Some(window.clone()),

            config,

            universe_simulation,

            controls: Some(controls),

            ui_system: Some(ui_system),
            camera_system,
            universe_simulation_updater_system,
            rendering_system: Some(rendering_system),
            physics_system,
            ui_cursor_system: Some(ui_cursor_system),
            ui_raycast_system: Some(ui_raycast_system),
            ui_raycast_result: vec![],

            remote_game_mode,
            current_camera: Camera::new(),
        }
    }

    pub fn new_headless() -> Self {
        let config = ResolutionConfig {
            width: 640,
            height: 480,
        };

        let mut universe_simulation = Simulation::new();

        universe_simulation
            .add_hierarchy(
                &config,
                &load_body_data("media/universe/solar_system/sun/sun.json"),
                None,
            )
            .expect("Failed to load sun.json");

        let universe_simulation_updater_system = UniverseSimulationUpdaterSystem::new();

        let camera_system = CameraSystem::new();
        let physics_system = PhysicsSystem::new();

        let remote_game_mode = RemoteGameMode::new();

        Self {
            // toolkit: None,
            window: None,

            config,

            universe_simulation,

            controls: None,

            ui_system: None,
            camera_system,
            universe_simulation_updater_system,
            rendering_system: None,
            physics_system,
            ui_cursor_system: None,
            ui_raycast_system: None,
            ui_raycast_result: vec![],

            remote_game_mode,
            current_camera: Camera::new(),
        }
    }

    pub fn measure_text_pixels(&self, text: &str, font_size: &UIFontSize) -> DVec2 {
        match &self.ui_system {
            None => DVec2::ZERO,
            Some(ui_system) => ui_system
                .ui_drawer
                .lock()
                .unwrap()
                .measure_text_pixels(text, font_size),
        }
    }

    pub fn update(&mut self) {
        profile!("before update", {
            self.remote_game_mode.ecs.time_counter.update_time();
            if let Some(ref mut controls) = self.controls {
                controls.update_gamepad_helper();

                if controls
                    .get_new_events()
                    .contains(&ControlEvent::ControlActivate(
                        ControlMapItem::RecompileShaders,
                    ))
                    && let Some(ref mut rendering_system) = self.rendering_system
                {
                    rendering_system.recreate_stages().unwrap();
                }
            }
        });

        if let Some(ref mut rendering_system) = self.rendering_system {
            self.remote_game_mode.update(
                &mut self.universe_simulation,
                &mut self.physics_system,
                rendering_system,
            );
        }

        if let Some(ref mut controls) = self.controls {
            controls.clear_events();
        }

        let stage_ecs = &mut self.remote_game_mode.ecs;

        profile!("universe_simulation_updater_system update", {
            self.universe_simulation_updater_system.update(
                &mut self.universe_simulation,
                stage_ecs,
                stage_ecs.time_counter.delta_time,
            );
        });

        if let Some(ref mut rendering_system) = self.rendering_system {
            profile!("physics_system update", {
                self.physics_system.update(
                    stage_ecs,
                    &self.universe_simulation,
                    rendering_system, // TODO how to do it without rendering
                    stage_ecs.time_counter.delta_time,
                );
            });
        }
        if let Some(ref mut controls) = self.controls {
            profile!("camera_system update", {
                self.camera_system
                    .update(&mut self.current_camera, controls, stage_ecs);
            });
        }

        if let Some(ref mut controls) = self.controls
            && let Some(ref window) = self.window
            && let Some(ref mut ui_cursor_system) = self.ui_cursor_system
            && let Some(ref mut ui_raycast_system) = self.ui_raycast_system
            && let Some(ref mut ui_system) = self.ui_system
        {
            profile!("ui_system & stuff update", {
                let window_size = window.lock().unwrap().inner_size();

                let normalized_cursor_pos = controls.mouse.get_cursor_absolute()
                    / DVec2::new(window_size.width as f64, window_size.height as f64);

                ui_system.update(stage_ecs, normalized_cursor_pos);
                ui_raycast_system.update(
                    &mut self.ui_raycast_result,
                    stage_ecs,
                    normalized_cursor_pos,
                );
                let cursor_system_result =
                    ui_cursor_system.update(stage_ecs, &self.ui_raycast_result);
                match cursor_system_result.cursor_locked {
                    true => {
                        if !controls.mouse.is_cursor_locked() {
                            controls.mouse.lock_cursor();
                        }
                    }
                    false => {
                        if controls.mouse.is_cursor_locked() {
                            controls.mouse.unlock_cursor();
                        }
                    }
                }
                match cursor_system_result.cursor_type {
                    UICursorType::Arrow => {
                        if controls.mouse.get_cursor_type() != CursorIcon::Default {
                            controls.mouse.set_cursor_type(CursorIcon::Default)
                        }
                    }
                    UICursorType::Pointer => {
                        if controls.mouse.get_cursor_type() != CursorIcon::Pointer {
                            controls.mouse.set_cursor_type(CursorIcon::Pointer)
                        }
                    }
                    UICursorType::Grab => {
                        if controls.mouse.get_cursor_type() != CursorIcon::Pointer {
                            controls.mouse.set_cursor_type(CursorIcon::Pointer)
                        }
                    }
                }
            });
        }

        if let Some(ref mut rendering_system) = self.rendering_system
            && let Some(ref ui_system) = self.ui_system
        {
            profile!("rendering_system update", {
                rendering_system.update(
                    stage_ecs,
                    &self.universe_simulation,
                    &self.current_camera,
                    ui_system,
                );
            });
        }
    }
}
