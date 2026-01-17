use common_util::strip_json_line_comments::strip_json_line_comments;
use dashu_float::DBig;
use glam::DVec3;
use math::decimal_vector_3d::DecimalVector3d;
use serde::{Deserialize, Serialize};
use std::fs;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyHeightModifier {
    pub image_path: String,
    #[ts(type = "[number, number, number]")]
    pub direction: DVec3,
    pub size: f64,
    pub rotation: f64,
    pub influence: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyColorModifier {
    pub image_path: String,
    #[ts(type = "[number, number, number]")]
    pub direction: DVec3,
    pub size: f64,
    pub rotation: f64,
    pub influence: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
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

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum BodyBiomeModifier {
    Latitude,
    Tidal,
    Random,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyBiome {
    pub id: u32,
    pub seed: f64,
    pub min_altitude: f64,
    pub max_altitude: f64,
    pub min_modifier: f64,
    pub max_modifier: f64,
    #[ts(type = "[number, number, number]")]
    pub color: DVec3,
    pub roughness: f64,
    pub erosion_strength: f64,
    pub deposition_strength: f64,
    pub craters_probability: f64,
    pub min_crater_size: f64,
    pub max_crater_size: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyTerrain {
    pub radius: f64,
    pub min_height: f64,
    pub max_height: f64,
    pub biome_modifier: BodyBiomeModifier,
    pub biomes: Vec<BodyBiome>,
    pub terrain_generation: BodyTerrainGeneration,
    pub icosphere_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyWater {
    pub radius: f64,
    pub waves_height: f64,
    #[ts(type = "[number, number, number]")]
    pub color: DVec3,
    pub icosphere_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyClouds {
    pub min_height: f64,
    pub max_height: f64,

    #[ts(type = "[number, number, number]")]
    pub color: DVec3,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyAtmosphere {
    pub seed: f64,
    pub start: f64,

    pub rayleigh_height: f64,
    pub rayleigh_density: f64,

    pub mie_height: f64,
    pub mie_density: f64,
    #[ts(type = "[number, number, number]")]
    pub mie_color: DVec3,

    pub clouds: Option<BodyClouds>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct StaticBodyMotion {
    pub position: DecimalVector3d,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct OrbitingBodyMotion {
    #[ts(type = "string")]
    pub orbit_radius: DBig,
    pub orbit_plane_normal: DecimalVector3d,
    #[ts(type = "string")]
    pub orbit_period: DBig,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum BodyMotion {
    Static(StaticBodyMotion),
    Orbiting(OrbitingBodyMotion),
}

fn empty_sat_vec() -> Vec<BodyCelestialBodyDefinition> {
    vec![]
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyDynamics {
    pub rotation_axis: DecimalVector3d,
    pub rotation_period: u64, // in seconds
    #[ts(type = "string")]
    pub mass: DBig, // in kg
    pub motion: BodyMotion,
    pub satellite_paths: Vec<String>,

    #[serde(default = "empty_sat_vec")]
    pub satellites: Vec<BodyCelestialBodyDefinition>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyPlanetGenConfig {
    pub out_dir: String,

    pub erosion_iterations: u16,
    pub erosion_droplets_count: u16,
    pub erosion_droplet_velocity_coefficient: f64,
    pub erosion_droplet_evaporation_coefficient: f64,

    pub cube_map_resolution: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyStarEmission {
    pub radius: f64,
    #[ts(type = "[number, number, number]")]
    pub radiance: DVec3,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BodyCelestialBodyDefinition {
    pub name: String,
    pub terrain: Option<BodyTerrain>,
    pub water: Option<BodyWater>,
    pub atmosphere: Option<BodyAtmosphere>,
    pub generator_config: Option<BodyPlanetGenConfig>,
    pub dynamics: BodyDynamics,
    pub star_emission: Option<BodyStarEmission>,
}

fn parse_body_data(str: &str) -> BodyCelestialBodyDefinition {
    let data: BodyCelestialBodyDefinition =
        serde_json::from_str(&strip_json_line_comments(str)).unwrap();
    data
}

pub fn load_body_data(path: &str) -> BodyCelestialBodyDefinition {
    println!("Loading body from path {}", path);
    let input_json = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to to read the input file {}", path));
    let mut data = parse_body_data(&input_json);
    for path in &data.dynamics.satellite_paths {
        data.dynamics.satellites.push(load_body_data(path.as_str()));
    }
    data
}
