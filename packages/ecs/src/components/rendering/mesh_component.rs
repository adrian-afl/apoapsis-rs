use crate::component_trait::acquire_next_id;
use glam::{DVec3, dvec3};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ScaledTextureDescription {
    pub texture_path: String,
    pub scale: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum ColorOrTextureDescription {
    #[ts(type = "[number, number, number]")]
    Color(DVec3),
    Texture(ScaledTextureDescription),
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum ValueOrTextureDescription {
    Value(f64),
    Texture(ScaledTextureDescription),
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDescription {
    pub color: ColorOrTextureDescription,
    pub roughness: ValueOrTextureDescription,
    pub metalness: ValueOrTextureDescription,
    pub emission: ColorOrTextureDescription,
    pub normal: Option<ScaledTextureDescription>,
    pub bump: Option<ScaledTextureDescription>,
}

impl MaterialDescription {
    pub fn default() -> Self {
        Self {
            color: ColorOrTextureDescription::Color(dvec3(0.0, 0.0, 0.0)),
            emission: ColorOrTextureDescription::Color(dvec3(0.0, 0.0, 0.0)),
            roughness: ValueOrTextureDescription::Value(1.0),
            metalness: ValueOrTextureDescription::Value(0.0),
            normal: None,
            bump: None,
        }
    }

    pub fn color_solid(mut self, color: DVec3) -> Self {
        self.color = ColorOrTextureDescription::Color(color);
        self
    }

    pub fn color_texture(mut self, path: &str) -> Self {
        self.color = ColorOrTextureDescription::Texture(ScaledTextureDescription {
            texture_path: path.to_owned(),
            scale: 1.0,
        });
        self
    }

    pub fn color_texture_scaled(mut self, path: &str, scale: f64) -> Self {
        self.color = ColorOrTextureDescription::Texture(ScaledTextureDescription {
            texture_path: path.to_owned(),
            scale,
        });
        self
    }

    pub fn emission_solid(mut self, color: DVec3) -> Self {
        self.emission = ColorOrTextureDescription::Color(color);
        self
    }

    pub fn emission_texture(mut self, path: &str) -> Self {
        self.emission = ColorOrTextureDescription::Texture(ScaledTextureDescription {
            texture_path: path.to_owned(),
            scale: 1.0,
        });
        self
    }

    pub fn emission_texture_scaled(mut self, path: &str, scale: f64) -> Self {
        self.emission = ColorOrTextureDescription::Texture(ScaledTextureDescription {
            texture_path: path.to_owned(),
            scale,
        });
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct MeshDescription {
    pub geometry_path: String,
    pub material: MaterialDescription,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct MeshComponent {
    pub id: u64,
    pub description: MeshDescription,
}

impl MeshComponent {
    pub fn from_description(description: MeshDescription) -> Self {
        Self {
            id: acquire_next_id(),
            description,
        }
    }
}
