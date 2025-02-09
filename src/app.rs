use core::game::Game;
use core::stages::warmup_stage::WarmupStage;
use game_stages::splash_screen_stage::SplashScreenStage;
use game_stages::stage_factory::StageFactory;
use glam::DVec2;
use std::sync::{Arc, Mutex};
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::event::{DeviceEvent, DeviceId, ElementState, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::window::Window;

pub struct GameWindowApp {
    game: Game,
}

impl GameWindowApp {
    pub fn new(toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>) -> GameWindowApp {
        let mut game = Game::new(toolkit, window);

        let initial_stage = Box::new(SplashScreenStage::new());
        game.push_game_stage(initial_stage);
        game.push_game_stage(Box::new(WarmupStage::new()));
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
                MouseScrollDelta::LineDelta(_, _) => (),
                MouseScrollDelta::PixelDelta(delta) => {
                    self.game.controls.mouse.on_mouse_scroll(delta.y)
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
            DeviceEvent::MouseWheel { .. } => {}
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Key(_) => {}
            _ => (),
        }
    }
}
