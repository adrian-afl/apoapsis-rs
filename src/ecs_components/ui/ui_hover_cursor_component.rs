use crate::ecs_components::ui::ui_cursor_component::UICursorType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIHoverCursorComponent {
    pub id: u64,
    typ: UICursorType,
}
