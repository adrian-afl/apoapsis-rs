use crate::component_trait::acquire_next_id;
use crate::components::ui::cursor_type::UICursorType;
use crate::components::ui::ui_hover_color_component::UIHoverColorComponent;
use glam::DVec4;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIHoverCursorComponent {
    pub id: u64,
    pub typ: UICursorType,
}

impl UIHoverCursorComponent {
    pub fn new(typ: UICursorType) -> Self {
        Self {
            id: acquire_next_id(),
            typ,
        }
    }
}
