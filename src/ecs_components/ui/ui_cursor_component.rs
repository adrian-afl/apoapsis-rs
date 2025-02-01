use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UICursorType {
    Arrow,
    Grab,
    Pointer,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UICursorComponent {
    pub id: u64,
    typ: UICursorType,
}
