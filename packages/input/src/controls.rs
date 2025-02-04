use crate::controls_mapping::{ControlMapItem, ControlsMapping};
use crate::mouse_input::MouseInput;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use winit::event::MouseButton;
use winit::keyboard::PhysicalKey;
use winit::window::Window;

#[derive(Clone, Debug)]
pub enum ControlEvent {
    ControlActivate(ControlMapItem),
    ControlRelease(ControlMapItem),
}

pub struct Controls {
    mapping: ControlsMapping,
    new_events: Vec<ControlEvent>,
    pub mouse: MouseInput,
    controls_state: HashMap<ControlMapItem, bool>,
}

impl Controls {
    pub fn new(window: Arc<Mutex<Window>>) -> Self {
        Self {
            new_events: Vec::new(),
            mouse: MouseInput::new(window.clone()),
            mapping: ControlsMapping::new(),
            controls_state: HashMap::new(),
        }
    }

    pub fn get_control_state(&self, control: ControlMapItem) -> bool {
        *self.controls_state.get(&control).unwrap_or(&false)
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: bool) {
        let mapped = self.mapping.map_mouse_button_event(button, state);
        for event in mapped {
            self.handle_control_event(event);
        }
    }

    pub fn on_key(&mut self, key: PhysicalKey, state: bool) {
        let mapped = self.mapping.map_keyboard_event(key, state);
        for event in mapped {
            self.handle_control_event(event);
        }
    }

    fn handle_control_event(&mut self, event: ControlEvent) {
        let (item, state) = match &event {
            ControlEvent::ControlActivate(item) => (item, true),
            ControlEvent::ControlRelease(item) => (item, false),
        };
        match self.controls_state.get_mut(&item) {
            None => {
                self.controls_state.insert(item.clone(), state);
            }
            Some(current) => *current = state,
        }
        self.new_events.push(event);
    }

    pub fn consume_new_events(&mut self) -> Vec<ControlEvent> {
        std::mem::take(&mut self.new_events)
    }
}
