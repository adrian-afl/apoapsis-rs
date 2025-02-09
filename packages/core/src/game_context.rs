use crate::time_counter::TimeCounter;
use ecs::components::ui::ui_text_component::UIFontSize;
use glam::DVec2;
use input::controls::Controls;
use ui_renderer::ui_system::UISystem;
use universe_simulation::simulation::Simulation;

pub struct GameContext<'a> {
    ui_system: &'a UISystem,
    pub total_time: f64,
    pub delta_time: f64,
    pub controls: &'a Controls,
    pub universe: &'a Simulation,
}

impl<'a> GameContext<'a> {
    pub fn new(
        ui_system: &'a UISystem,
        time_counter: &TimeCounter,
        universe: &'a Simulation,
        controls: &'a Controls,
    ) -> Self {
        Self {
            ui_system,
            total_time: time_counter.total_time,
            delta_time: time_counter.delta_time,
            universe,
            controls,
        }
    }

    pub fn measure_text_pixels(&self, text: &str, font_size: &UIFontSize) -> DVec2 {
        self.ui_system
            .ui_drawer
            .lock()
            .unwrap()
            .measure_text_pixels(text, font_size)
    }
}
