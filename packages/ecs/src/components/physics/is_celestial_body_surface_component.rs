use crate::component_trait::acquire_next_id;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct IsCelestialBodySurfaceComponent {
    #[serde(skip, default = "acquire_next_id")]
    pub id: u64,
    pub body_name: String,
    pub index: u16,
}

impl IsCelestialBodySurfaceComponent {
    pub fn new(body_name: String, index: u16) -> Self {
        Self {
            id: acquire_next_id(),
            body_name,
            index,
        }
    }
}
