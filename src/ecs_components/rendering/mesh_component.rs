use crate::ecs::component_trait::ComponentTypes;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use glam::DVec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaledTextureDescription {
    pub texture_path: String,
    pub scale: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ColorOrTextureDescription {
    Color(DVec3),
    Texture(ScaledTextureDescription),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueOrTextureDescription {
    Value(f64),
    Texture(ScaledTextureDescription),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDescription {
    pub color: ColorOrTextureDescription,
    pub roughness: ValueOrTextureDescription,
    pub metalness: ValueOrTextureDescription,
    pub emission: ColorOrTextureDescription,
    pub normal: Option<ScaledTextureDescription>,
    pub bump: Option<ScaledTextureDescription>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshDescription {
    pub geometry_path: String,
    pub material: MaterialDescription,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MeshComponent {
    pub id: u64,
    pub description: MeshDescription,
}

impl_component!(MeshComponent);

impl MeshComponent {
    pub fn from_description(description: MeshDescription) -> Self {
        Self {
            id: acquire_next_id(),
            description,
        }
    }
}
