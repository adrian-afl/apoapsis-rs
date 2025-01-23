use crate::core::game::Game;
use std::sync::Arc;
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::event::{DeviceEvent, DeviceId, KeyEvent, WindowEvent};
use winit::window::Window;

pub struct GameWindowApp {
    game: Game,
}

impl GameWindowApp {
    pub fn new(toolkit: Arc<VEToolkit>) -> GameWindowApp {
        GameWindowApp {
            game: Game::new(toolkit),
        }
    }
}

impl App for GameWindowApp {
    fn draw(&mut self, window: &mut Window) {
        self.game.update(window);
    }

    fn on_window_event(&self, event: WindowEvent) {
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
                } => {}
            },
            WindowEvent::CursorMoved { position, .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { delta, .. } => {}
            WindowEvent::MouseInput { state, button, .. } => {}
            _ => (),
        }
    }

    fn on_device_event(&self, device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { .. } => {}
            DeviceEvent::MouseWheel { .. } => {}
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Key(_) => {}
            _ => (),
        }
    }
}
