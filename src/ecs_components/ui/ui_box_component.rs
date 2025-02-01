use glam::DVec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIRectangleComponent {
    pub id: u64,
    size: DVec2,
}
