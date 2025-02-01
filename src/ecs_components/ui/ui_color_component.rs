use glam::DVec4;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIColorComponent {
    pub id: u64,
    pub color: DVec4,
}
