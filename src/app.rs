use crate::core::game::Game;
use glam::DVec2;
use std::sync::{Arc, Mutex};
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::event::{
    DeviceEvent, DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent,
};
use winit::window::Window;

pub struct GameWindowApp {
    game: Game,
}

impl GameWindowApp {
    pub fn new(toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>) -> GameWindowApp {
        GameWindowApp {
            game: Game::new(toolkit, window),
        }
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
                } => if !repeat {},
            },
            WindowEvent::CursorMoved { position, .. } => self
                .game
                .mouse_input
                .on_mouse_move_on_surface(DVec2::new(position.x, position.y)),
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, _) => (),
                MouseScrollDelta::PixelDelta(delta) => {
                    self.game.mouse_input.on_mouse_scroll(delta.y)
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                self.game.mouse_input.on_mouse_button(
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

    fn on_device_event(&mut self, device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => self
                .game
                .mouse_input
                .on_mouse_move_anywhere(DVec2::new(delta.0, delta.1)),
            DeviceEvent::MouseWheel { .. } => {}
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Key(_) => {}
            _ => (),
        }
    }
}
