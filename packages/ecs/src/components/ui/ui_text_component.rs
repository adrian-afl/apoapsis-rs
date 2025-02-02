use crate::component_trait::acquire_next_id;
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

impl UITextComponent {
    pub fn new(content: &str, color: DVec4, font_size: UIFontSize) -> Self {
        Self {
            id: acquire_next_id(),
            content: content.to_owned(),
            color,
            font_size,
        }
    }
}
