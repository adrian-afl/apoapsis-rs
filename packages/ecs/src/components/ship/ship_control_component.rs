use crate::component_trait::acquire_next_id;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct ShipControlComponent {
    #[serde(skip, default = "acquire_next_id")]
    pub id: u64,
    pub linear_impulse_strength: f64,
    pub angular_impulse_strength: f64,
}

impl ShipControlComponent {
    pub fn new(linear_impulse_strength: f64, angular_impulse_strength: f64) -> Self {
        Self {
            id: acquire_next_id(),
            linear_impulse_strength,
            angular_impulse_strength,
        }
    }
}
