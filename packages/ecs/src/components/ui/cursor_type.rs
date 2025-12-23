use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub enum UICursorType {
    Arrow,
    Pointer,
    Grab,
}
