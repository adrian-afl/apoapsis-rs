use glam::{DVec2, dvec2};
use std::sync::{Arc, Mutex};
use winit::dpi::PhysicalPosition;
use winit::window::{CursorGrabMode, CursorIcon, Window};

pub struct MouseInput {
    window: Arc<Mutex<Window>>,
    cursor_locked: bool,
    absolute_cursor_pos: DVec2,
    integrated_cursor_pos: DVec2,
    integrated_scroll: f64,
    cursor_icon: CursorIcon,
}

impl MouseInput {
    pub fn new(window: Arc<Mutex<Window>>) -> Self {
        Self {
            window,
            cursor_locked: false,
            absolute_cursor_pos: DVec2::new(0.0, 0.0),
            integrated_cursor_pos: DVec2::new(0.0, 0.0),
            integrated_scroll: 0.0,
            cursor_icon: CursorIcon::Default,
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

    pub fn set_cursor_type(&mut self, cursor_icon: CursorIcon) {
        let window = self.window.lock().unwrap();
        window.set_cursor(cursor_icon);
        self.cursor_icon = cursor_icon;
    }

    pub fn get_cursor_type(&self) -> CursorIcon {
        self.cursor_icon
    }

    pub fn is_cursor_locked(&self) -> bool {
        self.cursor_locked
    }

    pub fn get_cursor_absolute(&self) -> DVec2 {
        self.absolute_cursor_pos
    }

    pub fn get_cursor_integrated(&self) -> DVec2 {
        self.integrated_cursor_pos
    }

    pub fn get_scroll_integrated(&self) -> f64 {
        self.integrated_scroll
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
}
