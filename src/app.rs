use crate::cli_args::{CLIArgs, EntrypointOverride};
use core::game::Game;
use core::game_stage_trait::GameStage;
use core::stages::warmup_stage::WarmupStage;
use game_stages::body_viewer_stage::BodyViewerStage;
use game_stages::gaming_stage;
use game_stages::gaming_stage::gaming_initialize_sandbox_in_orbit::gaming_initialize_sandbox_in_orbit;
use game_stages::gaming_stage::gaming_stage::GamingStage;
use game_stages::splash_screen_stage::SplashScreenStage;
use glam::DVec2;
use remote::remote_controlled_game_stage::RemoteControlledGameStage;
use std::sync::{Arc, Mutex};
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::event::{DeviceEvent, DeviceId, ElementState, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::window::Window;

pub struct GameWindowApp {
    game: Game,
}

impl GameWindowApp {
    pub fn new(
        toolkit: Arc<VEToolkit>,
        window: Arc<Mutex<Window>>,
        cli_args: Arc<CLIArgs>,
    ) -> GameWindowApp {
        let mut game = Game::new(toolkit, window);

        match &cli_args.entry {
            None => {
                // let initial_stage = Box::new(SplashScreenStage::new());
                let initial_stage = Box::new(RemoteControlledGameStage::new(&game.get_context()));
                game.push_game_stage(initial_stage);
                game.push_game_stage(Box::new(WarmupStage::new()));
            }
            Some(entry) => match entry {
                EntrypointOverride::BodyViewer { name } => {
                    let stage = Box::new(BodyViewerStage::new(&game.get_context(), name));
                    game.push_game_stage(stage);
                }
                EntrypointOverride::OnGroundSandbox => panic!("Not implemented"),
                EntrypointOverride::InOrbitSandbox => {
                    let context = game.get_context();
                    let mut stage = Box::new(GamingStage::new(&context));
                    game.update_with(stage.get_ecs_world());
                    let context = game.get_context();
                    gaming_initialize_sandbox_in_orbit(&context, stage.get_ecs_world());
                    game.push_game_stage(stage);
                }
                EntrypointOverride::LoadSave { .. } => panic!("Not implemented"),
            },
        };

        GameWindowApp { game }
    }
}

impl App for GameWindowApp {
    fn draw(&mut self) {
        self.game.update();
    }

    fn on_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {}
            WindowEvent::Destroyed => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { event, .. } => match event {
                KeyEvent {
                    state,
                    physical_key,
                    repeat,
                    ..
                } => {
                    if !repeat {
                        self.game.controls.on_key(
                            physical_key,
                            match state {
                                ElementState::Pressed => true,
                                ElementState::Released => false,
                            },
                        )
                    }
                }
            },
            WindowEvent::CursorMoved { position, .. } => self
                .game
                .controls
                .mouse
                .on_mouse_move_on_surface(DVec2::new(position.x, position.y)),
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    // println!("Ecentr 1, x{}, y{}", x, y);
                    self.game.controls.mouse.on_mouse_scroll(y as f64)
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    // println!("Ecentr 2, x{}, y{}", delta.x, delta.y);
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                self.game.controls.on_mouse_button(
                    button,
                    match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    },
                );
            }
            _ => (),
        }
    }

    fn on_device_event(&mut self, _: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => self
                .game
                .controls
                .mouse
                .on_mouse_move_anywhere(DVec2::new(delta.0, delta.1)),
            DeviceEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    // println!("Ecentr 3, x{}, y{}", x, y);
                }
                MouseScrollDelta::PixelDelta(px) => {
                    // println!("Ecentr 4, x{}, y{}", px.x, px.y);
                }
            },
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Key(_) => {}
            _ => (),
        }
    }
}
