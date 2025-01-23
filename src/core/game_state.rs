use crate::math::sin_cos::f64_to_dbig;
use dashu_float::DBig;
use std::ops::Add;
use std::time::SystemTime;

pub struct GameState {
    pub current_game_time: DBig,

    start_time: f64,
    last_time: f64,
    elapsed: f64,
    delta_time: f64,
}

impl GameState {
    pub fn new() -> Self {
        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        Self {
            start_time,
            last_time: start_time,
            elapsed: 0.0,
            delta_time: 0.0,

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
