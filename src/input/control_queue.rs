use winit::event::MouseButton;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Clone, Debug)]
pub enum ControlEvent {
    Pause,
}

pub struct ControlQueue {
    events: Vec<ControlEvent>,
}

impl ControlQueue {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: bool) {
        match button {
            MouseButton::Left => {}
            MouseButton::Right => {}
            MouseButton::Middle => {}
            MouseButton::Back => {}
            MouseButton::Forward => {}
            MouseButton::Other(_) => {}
        }
    }

    pub fn on_key(&mut self, key: PhysicalKey, state: bool) {
        match key {
            PhysicalKey::Code(key) => match key {
                KeyCode::Escape => {
                    if state {
                        self.events.push(ControlEvent::Pause)
                    }
                }
                _ => (),
            },
            PhysicalKey::Unidentified(_) => (),
        }
    }

    pub fn get_events(&self) -> Vec<ControlEvent> {
        self.events.clone()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
