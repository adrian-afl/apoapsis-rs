use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct TimeCounter {
    start_time: f64,
    last_time: f64,
    pub total_time: f64,
    pub delta_time: f64,
}

impl TimeCounter {
    pub fn new() -> Self {
        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        Self {
            start_time,
            last_time: start_time,
            total_time: 0.0,
            delta_time: 0.0,
        }
    }

    pub fn update_time(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let delta_time = now - self.last_time;
        self.last_time = now;
        self.delta_time = delta_time;
        self.total_time += delta_time;
    }
}
