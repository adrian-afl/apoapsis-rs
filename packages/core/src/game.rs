use crate::camera_system::CameraSystem;
use crate::game_stage::{GameStage, StageTransition};
use crate::stages::splash_screen_stage::SplashScreenStage;
use crate::stages::stages_stack::StageStack;
use crate::time_counter::TimeCounter;
use celestial_renderer::renderer::Renderer;
use celestial_renderer::rendering_system::RenderingSystem;
use input::controls::Controls;
use real_physics_engine::physics_system::PhysicsSystem;
use renderer_common::camera::Camera;
use renderer_common::resolution_config::ResolutionConfig;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use ui_renderer::ui_system::UISystem;
use universe_simulation::body_definitions::load_body_data;
use universe_simulation::simulation::Simulation;
use universe_simulation::universe_simulation_updater_system::UniverseSimulationUpdaterSystem;
use vengine_rs::core::toolkit::VEToolkit;
use winit::window::Window;

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
        stage_stack.push(Box::new(SplashScreenStage::new()));

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

            stage_stack,
            current_camera: Camera::new(),
        }
    }

    pub fn update(&mut self) {
        self.time_counter.update_time();

        let mut transition_from_update = StageTransition::DoNothing;
        let mut transition_from_controls = StageTransition::DoNothing;

        if let Some(stage) = &self.stage_stack.head() {
            let mut stage = stage.lock().unwrap();
            transition_from_update =
                stage.update(self.time_counter.total_time, self.time_counter.delta_time);
            transition_from_controls = stage.handle_controls(&mut self.controls);

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

            self.ui_system.update(stage_ecs);

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

        match transition_from_controls {
            StageTransition::PushStage(stage) => self.stage_stack.push(stage),
            StageTransition::PopSelf => {
                self.stage_stack.pop();
            }
            StageTransition::DoNothing => (),
        }
    }
}
