use crate::remote_api::remote_game_mode::RemoteGameExecutionContext;
use crate::remote_api::util::serde_parse_err_map;
use glam::DVec3;
use math::decimal_vector_3d::DecimalVector3d;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

// @api_command get_debug_real_physics_wireframe(): DebugCollector
pub fn get_debug_real_physics_wireframe(
    _payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let data = context.physics_system.debug_get_world();

    Ok(Some(json!(data).to_string()))
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct RaycastRealPhysicsInput {
    #[ts(type = "[number, number, number]")]
    point: DVec3,
    #[ts(type = "[number, number, number]")]
    direction: DVec3,
}

// @api_command raycast_real_physics(point: DVec3, direction: DVec3): number | null
pub fn raycast_real_physics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: RaycastRealPhysicsInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;
    let data = context
        .physics_system
        .raycast_real(input.point, input.direction);

    Ok(Some(json!(data).to_string()))
}
