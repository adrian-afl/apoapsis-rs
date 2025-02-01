use glam::DVec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIBoxComponent {
    pub id: u64,
    pub size: DVec2,
    pub position: DVec2,
    pub orientation: f64, // radians
    pub z_index: i32,
}
