use crate::time_counter::TimeCounter;
use celestial_renderer::rendering_system::RenderingSystem;
use ecs::components::ui::ui_text_component::UIFontSize;
use glam::{DVec2, DVec3};
use input::controls::Controls;
use math::decimal_vector_3d::DecimalVector3d;
use ui_renderer::ui_system::UISystem;
use universe_simulation::simulation::Simulation;

pub struct GameContext<'a> {
    ui_system: &'a UISystem,
    rendering_system: &'a RenderingSystem,
    pub total_time: f64,
    pub delta_time: f64,
    pub controls: &'a Controls,
    pub universe: &'a Simulation,
}

impl<'a> GameContext<'a> {
    pub fn new(
        ui_system: &'a UISystem,
        rendering_system: &'a RenderingSystem,
        time_counter: &TimeCounter,
        universe: &'a Simulation,
        controls: &'a Controls,
    ) -> Self {
        Self {
            ui_system,
            rendering_system,
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

    pub fn get_altitude(&self, point: &DecimalVector3d) -> Option<f64> {
        self.rendering_system.get_altitude(self.universe, &point)
    }

    pub fn get_terrain_distance_from_center(&self, body: &str, normal: DVec3) -> Option<f64> {
        self.rendering_system
            .get_terrain_distance_from_center(self.universe, body, normal)
    }

    pub fn get_water_distance_from_center(&self, body: &str, normal: DVec3) -> Option<f64> {
        self.rendering_system
            .get_water_distance_from_center(self.universe, body, normal)
    }
}
