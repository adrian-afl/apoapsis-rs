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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetRealAltitudeOverCelestialBody {
    name: String,
    point: DecimalVector3d,
}

// @XDapi_command get_real_altitude_over_celestial_body(): {water: number, terrain:}
// pub fn get_all_celestial_body_names(
//     payload: &str,
//     context: &mut RemoteGameExecutionContext,
// ) -> Result<Option<String>, String> {
//     let names: Vec<String> = context
//         .simulation
//         .bodies
//         .iter()
//         .map(|x| x.body.name.clone())
//         .collect();
//
//     Ok(Some(json!(names).to_string()))
// }
