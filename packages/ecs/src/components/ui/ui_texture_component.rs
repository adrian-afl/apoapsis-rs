use crate::component_trait::acquire_next_id;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct UITextureComponent {
    pub id: u64,
    pub texture_path: String,
}

impl UITextureComponent {
    pub fn new(texture_path: &str) -> Self {
        Self {
            id: acquire_next_id(),
            texture_path: texture_path.to_owned(),
        }
    }
}
