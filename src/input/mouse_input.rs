use crate::input::control_queue::ControlQueue;
use glam::DVec2;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use winit::dpi::PhysicalPosition;
use winit::event::MouseButton;
use winit::window::{CursorGrabMode, Window};

pub struct MouseInput {
    window: Arc<Mutex<Window>>,
    control_queue: Arc<Mutex<ControlQueue>>,
    button_state: HashMap<MouseButton, bool>,
    cursor_locked: bool,
    absolute_cursor_pos: DVec2,
    integrated_cursor_pos: DVec2,
    integrated_scroll: f64,
}

impl MouseInput {
    pub fn new(window: Arc<Mutex<Window>>, control_queue: Arc<Mutex<ControlQueue>>) -> Self {
        Self {
            window,
            control_queue,
            cursor_locked: false,
            button_state: HashMap::new(),
            absolute_cursor_pos: DVec2::new(0.0, 0.0),
            integrated_cursor_pos: DVec2::new(0.0, 0.0),
            integrated_scroll: 0.0,
        }
    }

    pub fn lock_cursor(&mut self) {
        let window = self.window.lock().unwrap();

        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "linux"))]
        {
            window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
        }
        #[cfg(any(target_os = "windows"))]
        {
            window.set_cursor_grab(CursorGrabMode::Confined).unwrap();
        }
        window.set_cursor_visible(false);
        self.cursor_locked = true;
    }

    pub fn unlock_cursor(&mut self) {
        let window = self.window.lock().unwrap();
        window.set_cursor_grab(CursorGrabMode::None).unwrap();
        window.set_cursor_visible(true);
        self.cursor_locked = false;
    }

    pub fn is_cursor_locked(&self) -> bool {
        self.cursor_locked
    }

    pub fn on_mouse_move_on_surface(&mut self, absolute_position: DVec2) {
        if !self.cursor_locked {
            self.absolute_cursor_pos = absolute_position;
        }
    }

    pub fn on_mouse_move_anywhere(&mut self, delta_position: DVec2) {
        if self.cursor_locked {
            self.integrated_cursor_pos += delta_position;
            let window = self.window.lock().unwrap();
            let size = window.inner_size();
            window
                .set_cursor_position(PhysicalPosition::new(size.width / 2, size.height / 2))
                .unwrap();
        }
    }

    pub fn on_mouse_scroll(&mut self, delta: f64) {
        self.integrated_scroll += delta;
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: bool) {
        match self.button_state.get_mut(&button) {
            None => {
                self.button_state.insert(button, state);
            }
            Some(current) => *current = state,
        }
        self.control_queue
            .lock()
            .unwrap()
            .on_mouse_button(button, state);
    }
}
