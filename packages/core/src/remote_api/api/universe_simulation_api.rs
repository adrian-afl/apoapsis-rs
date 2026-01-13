use crate::remote_api::remote_game_mode::RemoteGameExecutionContext;
use crate::remote_api::util::serde_parse_err_map;
use ecs::component_trait::{AttachedComponents, Components};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use math::decimal_vector_3d::DecimalVector3d;
use math::sin_cos::f64_to_dbig;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use universe_simulation::body_definitions::BodyTerrain;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddEntityInput {
    components: Option<AttachedComponents>,
}

// @api_command get_all_celestial_body_names(): string[]
pub fn get_all_celestial_body_names(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let names: Vec<String> = context
        .simulation
        .bodies
        .iter()
        .map(|x| x.body.name.clone())
        .collect();

    Ok(Some(json!(names).to_string()))
}

// @api_command get_celestial_body_position(name: string): DecimalVector3d
pub fn get_celestial_body_position(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let body_name = payload;
    let body = context.simulation.get_body(body_name);

    Ok(Some(json!(body.position).to_string()))
}

// @api_command get_celestial_body_surface_velocity(name: string, point: DecimalVector3d): DecimalVector3d
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetCelestialBodySurfaceVelocityInput {
    name: String,
    point: DecimalVector3d,
}

pub fn get_celestial_body_surface_velocity(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetCelestialBodySurfaceVelocityInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;
    let surface_velocity = context
        .simulation
        .get_surface_velocity(&input.name, &input.point);

    Ok(Some(json!(surface_velocity).to_string()))
}

// @api_command get_celestial_body_orientation(name: string): DecimalMatrix3d
pub fn get_celestial_body_orientation(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let body_name = payload;
    let body = context.simulation.get_body(body_name);

    Ok(Some(json!(body.orientation).to_string()))
}

// @api_command get_celestial_body_parent(name: string): string | null
pub fn get_celestial_body_parent(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let body_name = payload;
    let parent = context.simulation.get_body_parent(body_name);

    Ok(Some(json!(parent.map(|x| x.body.name.clone())).to_string()))
}

// @api_command get_celestial_body_satellites(name: string): string[]
pub fn get_celestial_body_satellites(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let body_name = payload;
    let sats: Vec<String> = context
        .simulation
        .get_body_satellites(body_name)
        .iter()
        .map(|x| x.body.name.clone())
        .collect();

    Ok(Some(json!(sats).to_string()))
}

// @api_command get_altitude_over_celestial_body(name: string, point: DecimalVector3d): string
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetAltitudeOverCelestialBodyInput {
    name: String,
    point: DecimalVector3d,
}

// TODO remember to use rendering_system here for precise measurement if its available
// now its just using the radiuses
pub fn get_altitude_over_celestial_body(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetAltitudeOverCelestialBodyInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;
    let body = context.simulation.get_body(&input.name);

    let radius: f64 = {
        let terrain_radius = match &body.body.terrain {
            None => 0.0,
            Some(terrain) => terrain.radius,
        };
        let water_radius = match &body.body.water {
            None => 0.0,
            Some(water) => water.radius,
        };
        if terrain_radius > water_radius {
            terrain_radius
        } else {
            water_radius
        }
    };

    Ok(Some(
        json!(body.position.distance_to(&input.point) - f64_to_dbig(radius)).to_string(),
    ))
}

// @api_command get_closest_celestial_body(point: DecimalVector3d): string
pub fn get_closest_celestial_body(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let point: DecimalVector3d = serde_json::from_str(payload).map_err(serde_parse_err_map)?;
    let body = context.simulation.find_closest_body(&point);

    Ok(Some(json!(body.body.name).to_string()))
}

// @api_command get_gravity_flux(point: DecimalVector3d): DecimalVector3d
pub fn get_gravity_flux(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let point: DecimalVector3d = serde_json::from_str(payload).map_err(serde_parse_err_map)?;
    let flux = context.simulation.calculate_gravity_flux(&point);

    Ok(Some(json!(flux).to_string()))
}
