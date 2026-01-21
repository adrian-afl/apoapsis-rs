use crate::component_trait::acquire_next_id;
use glam::DVec4;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct UIHoverColorComponent {
    #[serde(skip, default = "acquire_next_id")]
    pub id: u64,
    #[ts(type = "[number, number, number, number]")]
    pub color: DVec4,
}

impl UIHoverColorComponent {
    pub fn new(color: DVec4) -> Self {
        Self {
            id: acquire_next_id(),
            color,
        }
    }

    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            id: acquire_next_id(),
            color: DVec4::new(r, g, b, 1.0),
        }
    }

    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            id: acquire_next_id(),
            color: DVec4::new(r, g, b, a),
        }
    }
}
