use core::game::Game;
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
        let game = Game::new(toolkit, window);

        GameWindowApp { game }
    }
}

impl App for GameWindowApp {
    fn draw(&mut self) {
        self.game.update();
    }

    fn on_window_event(&mut self, event: WindowEvent) {
        if let Some(ref mut controls) = self.game.controls {
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
                            controls.on_key(
                                physical_key,
                                match state {
                                    ElementState::Pressed => true,
                                    ElementState::Released => false,
                                },
                            )
                        }
                    }
                },
                WindowEvent::CursorMoved { position, .. } => controls
                    .mouse
                    .on_mouse_move_on_surface(DVec2::new(position.x, position.y)),
                WindowEvent::CursorEntered { .. } => {}
                WindowEvent::CursorLeft { .. } => {}
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        // println!("Ecentr 1, x{}, y{}", x, y);
                        controls.mouse.on_mouse_scroll(y as f64)
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        // println!("Ecentr 2, x{}, y{}", delta.x, delta.y);
                    }
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    controls.on_mouse_button(
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
    }

    fn on_device_event(&mut self, _: DeviceId, event: DeviceEvent) {
        if let Some(ref mut controls) = self.game.controls {
            match event {
                DeviceEvent::MouseMotion { delta } => controls
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
}
