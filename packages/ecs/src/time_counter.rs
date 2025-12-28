use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TimeCounter {
    pub last_time: f64,
    pub total_time: f64,
    pub delta_time: f64,
}

impl Default for TimeCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeCounter {
    pub fn new() -> Self {
        Self {
            last_time: 0.0,
            total_time: 0.0,
            delta_time: 0.0,
        }
    }

    pub fn update_time(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        if self.last_time == 0.0 {
            self.last_time = now;
        } else {
            let delta_time = now - self.last_time;
            self.last_time = now;
            self.delta_time = delta_time;
            self.total_time += delta_time;
        }
    }

    pub fn reset(&mut self) {
        self.last_time = 0.0;
        self.total_time = 0.0;
        self.delta_time = 0.0;
    }
}
