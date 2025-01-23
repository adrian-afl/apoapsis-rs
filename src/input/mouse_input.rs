use std::collections::HashMap;
use glam::DVec2;

pub struct MouseInput {
    button_state: HashMap<u8, bool>,
    absolute_cursor_pos: DVec2,
    integrated_cursor_pos: DVec2,
    integrated_scroll: f64,
}

impl MouseInput {
    pub fn new() -> Self {
        Self {
            button_state: HashMap::new(),
            absolute_cursor_pos: DVec2::new(0.0, 0.0),
            integrated_cursor_pos: DVec2::new(0.0, 0.0),
            integrated_scroll: 0.0
        }
    }
    
    pub fn on_mouse_move_on_surface(
}
