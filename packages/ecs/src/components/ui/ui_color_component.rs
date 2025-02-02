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
}
