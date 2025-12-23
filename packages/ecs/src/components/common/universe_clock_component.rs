use crate::component_trait::acquire_next_id;
use dashu_float::DBig;
use math::sin_cos::f64_to_dbig;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct UniverseClockComponent {
    pub id: u64,
    #[ts(type = "string")]
    pub time: DBig,
    pub should_advance: bool,
}

impl UniverseClockComponent {
    pub fn new(start: DBig, should_advance: bool) -> Self {
        Self {
            id: acquire_next_id(),
            time: start,
            should_advance,
        }
    }

    pub fn advance(&mut self, seconds: f64) {
        if self.should_advance {
            self.time = &self.time + &f64_to_dbig(seconds);
        }
    }
}
