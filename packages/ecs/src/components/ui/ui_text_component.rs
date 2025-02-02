use glam::DVec4;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UIFontSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UITextComponent {
    pub id: u64,
    pub content: String,
    pub color: DVec4,
    pub font_size: UIFontSize,
}
