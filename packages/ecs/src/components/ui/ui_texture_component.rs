use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UITextureComponent {
    pub id: u64,
    pub texture_path: String,
}
