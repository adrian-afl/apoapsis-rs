use crate::ecs::component_trait::ComponentsEnum;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use glam::DVec3;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaledTextureDescription {
    pub texture_path: String,
    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ColorOrTextureDescription {
    Color(DVec3),
    Texture(ScaledTextureDescription),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueOrTextureDescription {
    Value(f64),
    Texture(ScaledTextureDescription),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDescription {
    pub color: ColorOrTextureDescription,
    pub roughness: ValueOrTextureDescription,
    pub metalness: ValueOrTextureDescription,
    pub emission: ColorOrTextureDescription,
    pub normal: Option<ScaledTextureDescription>,
    pub bump: Option<ScaledTextureDescription>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshDescription {
    pub geometry_path: String,
    pub material: MaterialDescription,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeshComponent {
    pub id: u64,
    pub description: MeshDescription,
}

impl_component!(MeshComponent, true);

impl MeshComponent {
    pub fn from_description(description: MeshDescription) -> Self {
        Self {
            id: acquire_next_id(),
            description,
        }
    }
}
