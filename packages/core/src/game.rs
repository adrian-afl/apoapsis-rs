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
use media_provider::generic_cache::GenericCache;
use rayon::join;
use real_physics_engine::physics_system::PhysicsSystem;
use renderer_common::camera::Camera;
use renderer_common::resolution_config::ResolutionConfig;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tcpapi::send_event;
use ts_rs::TS;
use ui_renderer::ui_cursor_system::UICursorSystem;
use ui_renderer::ui_raycast_system::{UIRaycastResultItem, UIRaycastSystem};
use ui_renderer::ui_system::UISystem;
use universe_simulation::body_definitions::load_body_data;
use universe_simulation::simulation::Simulation;
use universe_simulation::universe_simulation_updater_system::UniverseSimulationUpdaterSystem;
use vengine_rs::core::toolkit::VEToolkit;
use winit::window::{CursorIcon, Window};

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct OnGameBootReadyEventData {
    pub headless: bool,
}

// @api_event on_control_activate(ControlMapItem)
// @api_event on_control_release(ControlMapItem)
// @api_event on_raw_key_down(number)
// @api_event on_raw_key_up(number)
// @api_event on_raw_input_text(string)

pub struct Game {
    // toolkit: Arc<VEToolkit>,
    window: Arc<Mutex<Window>>,

    pub config: ResolutionConfig,

    universe_simulation: Simulation,

    pub controls: Controls,

    remote_game_mode: RemoteGameMode,

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

    debug_mode_value: f64,

    cache_f64: GenericCache<f64>,
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
            window: window.clone(),

            config,

            universe_simulation,

            controls: controls,

            ui_system: ui_system,
            camera_system,
            universe_simulation_updater_system,
            rendering_system: rendering_system,
            physics_system,
            ui_cursor_system: ui_cursor_system,
            ui_raycast_system: ui_raycast_system,
            ui_raycast_result: vec![],

            remote_game_mode,
            current_camera: Camera::new(),

            debug_mode_value: 0.0,

            cache_f64: GenericCache::new(1024),
        }
    }

    pub fn measure_text_pixels(&self, text: &str, font_size: &UIFontSize) -> DVec2 {
        self.ui_system
            .ui_drawer
            .lock()
            .unwrap()
            .measure_text_pixels(text, font_size)
    }

    pub fn update(&mut self) {
        profile!("before update", {
            self.remote_game_mode.ecs.time_counter.update_time();
            self.controls.update_gamepad_helper();
            let control_events = self.controls.get_new_events();

            if self
                .controls
                .get_new_events()
                .contains(&ControlEvent::ControlActivate(
                    ControlMapItem::RecompileShaders,
                ))
            {
                self.rendering_system.recreate_stages().unwrap();
            }

            for event in control_events {
                match event {
                    ControlEvent::ControlActivate(v) => {
                        send_event!("on_control_activate", v);
                    }
                    ControlEvent::ControlRelease(v) => {
                        send_event!("on_control_release", v);
                    }
                    ControlEvent::RawKeyDown(v) => {
                        send_event!("on_raw_key_down", v);
                    }
                    ControlEvent::RawKeyUp(v) => {
                        send_event!("on_raw_key_up", v);
                    }
                    ControlEvent::RawText(v) => {
                        send_event!("on_raw_input_text", v);
                    }
                }
            }
        });

        self.remote_game_mode.update(
            &mut self.universe_simulation,
            &mut self.physics_system,
            &mut self.rendering_system,
        );

        self.controls.clear_events();

        let stage_ecs = &mut self.remote_game_mode.ecs;

        profile!("universe_simulation_updater_system update", {
            self.universe_simulation_updater_system.update(
                &mut self.universe_simulation,
                stage_ecs,
                stage_ecs.time_counter.delta_time,
            );
        });

        profile!("physics_system update part 1", {
            self.physics_system.update_part_1(
                stage_ecs,
                &self.universe_simulation,
                &self.rendering_system, // TODO how to do it without rendering
                &self.cache_f64,
                stage_ecs.time_counter.delta_time,
            );
        });

        profile!("camera_system update", {
            self.camera_system
                .update(&mut self.current_camera, &self.controls, stage_ecs);
        });

        profile!("ui_system & stuff update", {
            let window_size = self.window.lock().unwrap().inner_size();

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
        });

        let dt = stage_ecs.time_counter.delta_time;
        join(
            || {
                profile!("rendering_system update", {
                    self.rendering_system.update(
                        stage_ecs,
                        &self.universe_simulation,
                        &self.current_camera,
                        &self.ui_system,
                        &self.cache_f64,
                        self.debug_mode_value,
                    );
                });
            },
            || {
                profile!("physics_system update part 2", {
                    self.physics_system.update_part_2_physics_step(dt);
                });
            },
        );

        profile!("physics_system update part 3", {
            self.physics_system.update_part_3(
                stage_ecs,
                &self.universe_simulation,
                &self.rendering_system, // TODO how to do it without rendering
                &self.cache_f64,
                stage_ecs.time_counter.delta_time,
            );
        });
    }
}
