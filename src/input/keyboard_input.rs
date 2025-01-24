use crate::input::control_queue::ControlQueue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use winit::keyboard::PhysicalKey;
use winit::window::Window;

pub struct KeyboardInput {
    window: Arc<Mutex<Window>>,
    control_queue: Arc<Mutex<ControlQueue>>,
    button_state: HashMap<PhysicalKey, bool>,
}

impl KeyboardInput {
    pub fn new(window: Arc<Mutex<Window>>, control_queue: Arc<Mutex<ControlQueue>>) -> Self {
        Self {
            window,
            control_queue,
            button_state: HashMap::new(),
        }
    }

    pub fn on_key(&mut self, key: PhysicalKey, state: bool) {
        match self.button_state.get_mut(&key) {
            None => {
                self.button_state.insert(key, state);
            }
            Some(current) => *current = state,
        }
        self.control_queue.lock().unwrap().on_key(key, state);
    }
}
