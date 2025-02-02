use glam::DVec3;
use vengine_rs::image::image::VEImage;

pub struct ScaledTexture {
    pub texture: VEImage,
    pub scale: f64,
}

pub enum ColorOrTexture {
    Color(DVec3),
    Texture(ScaledTexture),
}

pub enum ValueOrTexture {
    Value(f64),
    Texture(ScaledTexture),
}

pub struct Material {
    pub color: ColorOrTexture,
    pub roughness: ValueOrTexture,
    pub metalness: ValueOrTexture,
    pub emission: ColorOrTexture,
    pub normal: Option<ScaledTexture>,
    pub bump: Option<ScaledTexture>,
}
