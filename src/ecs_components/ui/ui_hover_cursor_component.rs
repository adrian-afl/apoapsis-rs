use crate::ecs_components::ui::cursor_type::UICursorType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIHoverCursorComponent {
    pub id: u64,
    pub typ: UICursorType,
}
