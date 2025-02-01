use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UICursorType {
    Arrow,
    Pointer,
    Grab,
}
