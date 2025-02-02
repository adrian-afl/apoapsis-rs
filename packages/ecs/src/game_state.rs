use dashu_float::DBig;
use math::sin_cos::f64_to_dbig;
use renderer_common::camera::Camera;
use std::ops::Add;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct GameState {
    pub current_game_time: DBig,

    pub current_camera: Camera,

    start_time: f64,
    last_time: f64,
    pub elapsed: f64,
    pub delta_time: f64,
}

impl GameState {
    pub fn new() -> Self {
        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut current_camera = Camera::new();

        current_camera.set_perspective(90.0, 640.0 / 480.0, 0.1, 100000000.0);

        Self {
            start_time,
            last_time: start_time,
            elapsed: 0.0,
            delta_time: 0.0,

            current_camera,

            current_game_time: DBig::ZERO.clone(),
        }
    }

    pub fn update_time(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let delta_time = now - self.last_time;
        self.last_time = now;

        self.current_game_time = (&self.current_game_time).add(f64_to_dbig(delta_time));
        self.delta_time = delta_time;
    }
}
