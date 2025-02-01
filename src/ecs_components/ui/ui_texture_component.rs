use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UITextureComponent {
    pub id: u64,
    texture_path: String,
}
