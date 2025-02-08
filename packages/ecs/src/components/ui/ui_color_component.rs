use crate::component_trait::acquire_next_id;
use glam::DVec4;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIColorComponent {
    pub id: u64,
    pub color: DVec4,
}

impl UIColorComponent {
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
