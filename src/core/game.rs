use crate::app::CelestialRendererApp;
use crate::celestial_rendering::scene::camera::Camera;
use crate::config::Config;

pub struct Game {
    start_time: f64,
    last_time: f64,

    pub config: Config,

    camera: Camera,

    renderer: CelestialRendererApp,
}
