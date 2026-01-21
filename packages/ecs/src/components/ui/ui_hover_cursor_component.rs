use crate::component_trait::acquire_next_id;
use crate::components::ui::cursor_type::UICursorType;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct UIHoverCursorComponent {
    #[serde(skip, default = "acquire_next_id")]
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
