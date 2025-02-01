use glam::DVec4;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIHoverColorComponent {
    pub id: u64,
    color: DVec4,
}
