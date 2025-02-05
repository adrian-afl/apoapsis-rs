use glam::Vec3;

#[derive(Clone)]
pub struct InterpolatedBiomeData {
    //   pub dominating_id: u32,
    pub color: Vec3,
    pub roughness: f32,
    pub erosion_strength: f32,
    pub deposition_strength: f32,
    pub craters_probability: f32,
    pub min_crater_size: f32,
    pub max_crater_size: f32,
}

#[derive(Clone, Default)]
pub struct LoadedBiomeData {
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub roughness: u8,
}
