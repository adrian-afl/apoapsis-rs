use crate::component_trait::acquire_next_id;
use glam::DVec4;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub enum UIFontSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct UITextComponent {
    pub id: u64,
    pub content: String,
    #[ts(type = "[number, number, number, number]")]
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
