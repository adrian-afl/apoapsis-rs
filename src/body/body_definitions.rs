use crate::math::decimal_vector_3d::DecimalVector3d;
use crate::math::deserializable_dbig::DeserializableDBig;
use glam::DVec3;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyHeightModifier {
    pub image_path: String,
    pub direction: DVec3,
    pub size: f64,
    pub rotation: f64,
    pub influence: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyColorModifier {
    pub image_path: String,
    pub direction: DVec3,
    pub size: f64,
    pub rotation: f64,
    pub influence: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyTerrainGeneration {
    pub seed: f64,
    pub fbm_scale: f64,
    pub fbm_iterations: u8,
    pub fbm_iteration_scale_coefficient: f64,
    pub fbm_iteration_weight_coefficient: f64,
    pub fbm_final_power: f64,
    pub height_modifiers: Vec<BodyHeightModifier>,
    pub color_modifiers: Vec<BodyColorModifier>,
    pub craters_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BodyBiomeModifier {
    Latitude,
    Tidal,
    Random,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyBiome {
    pub id: u32,
    pub seed: f64,
    pub min_altitude: f64,
    pub max_altitude: f64,
    pub min_modifier: f64,
    pub max_modifier: f64,
    pub color: DVec3,
    pub roughness: f64,
    pub erosion_strength: f64,
    pub deposition_strength: f64,
    pub craters_probability: f64,
    pub min_crater_size: f64,
    pub max_crater_size: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyTerrain {
    pub radius: f64,
    pub min_height: f64,
    pub max_height: f64,
    pub biome_modifier: BodyBiomeModifier,
    pub biomes: Vec<BodyBiome>,
    pub terrain_generation: BodyTerrainGeneration,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyWater {
    pub height: f64,
    pub waves_height: f64,
    pub color: DVec3,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyClouds {
    pub min_height: f64,
    pub max_height: f64,

    pub color: DVec3,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyAtmosphere {
    pub seed: f64,
    pub start: f64,

    pub rayleigh_height: f64,
    pub rayleigh_density: f64,

    pub mie_height: f64,
    pub mie_density: f64,
    pub mie_color: DVec3,

    pub clouds: Option<BodyClouds>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StaticBodyMotion {
    pub position: DecimalVector3d,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrbitingBodyMotion {
    pub orbit_radius: DeserializableDBig,
    pub orbit_plane_normal: DecimalVector3d,
    pub orbit_period: DeserializableDBig,
}

#[derive(Debug, Clone, Deserialize)]
pub enum BodyMotion {
    Static(StaticBodyMotion),
    Orbiting(OrbitingBodyMotion),
}

fn empty_sat_vec() -> Vec<BodyCelestialBodyDefinition> {
    vec![]
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyDynamics {
    pub name: String,
    pub rotation_axis: DecimalVector3d,
    pub rotation_period: u64,     // in seconds
    pub mass: DeserializableDBig, // in kg
    pub motion: BodyMotion,
    pub satellite_paths: Vec<String>,

    #[serde(default = "empty_sat_vec")]
    pub satellites: Vec<BodyCelestialBodyDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyPlanetGenConfig {
    pub out_dir: String,

    pub subdivide_initial: u8,
    pub subdivide_level1: u8,
    pub subdivide_level2: u8,
    pub subdivide_level3: u8,

    pub erosion_iterations: u16,
    pub erosion_droplets_count: u16,
    pub erosion_droplet_velocity_coefficient: f64,
    pub erosion_droplet_evaporation_coefficient: f64,

    pub cube_map_resolution: u16,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyCelestialBodyDefinition {
    pub name: String,
    pub terrain: Option<BodyTerrain>,
    pub water: Option<BodyWater>,
    pub atmosphere: Option<BodyAtmosphere>,
    pub generator_config: BodyPlanetGenConfig,
    pub dynamics: BodyDynamics,
}

fn parse_body_data(str: &str) -> BodyCelestialBodyDefinition {
    let data: BodyCelestialBodyDefinition = serde_json::from_str(str).unwrap();
    data
}

pub fn load_body_data(path: &str) -> BodyCelestialBodyDefinition {
    let input_json = fs::read_to_string(path).expect("Failed to to read the input file");
    let mut data = parse_body_data(&input_json);
    for path in &data.dynamics.satellite_paths {
        data.dynamics.satellites.push(load_body_data(path.as_str()));
    }
    data
}
