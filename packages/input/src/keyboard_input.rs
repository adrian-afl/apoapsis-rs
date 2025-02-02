use crate::controls::Controls;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use winit::keyboard::PhysicalKey;
use winit::window::Window;

pub struct KeyboardInput {
    window: Arc<Mutex<Window>>,
    controls: Arc<Controls>,
    button_state: HashMap<PhysicalKey, bool>,
}

impl KeyboardInput {
    pub fn new(window: Arc<Mutex<Window>>, controls: Arc<Controls>) -> Self {
        Self {
            window,
            controls,
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
        self.controls.on_key(key, state);
    }
}
