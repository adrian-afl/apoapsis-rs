use crate::remote_api::util::serde_parse_err_map;
use ecs::component_trait::{AttachedComponents, Components};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddEntityInput {
    components: Option<AttachedComponents>,
}

// @api_command get_all_celestial_body_names(): string[]
// @api_command get_celestial_body_position(string): DecimalVector3d
// @api_command get_celestial_body_surface_velocity(string, DecimalVector3d): DecimalVector3d
// @api_command get_celestial_body_orientation(string): DecimalMatrix3d
// @api_command get_celestial_body_parent(string): string | null
// @api_command get_celestial_body_satellites(string): string[]
// @api_command get_altitude_over_celestial_body(string): string
// @api_command get_closest_celestial_body(DecimalVector3d): string
// @api_command get_gravity_flux(DecimalVector3d): DecimalVector3d
