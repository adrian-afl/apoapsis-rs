use crate::remote_api::remote_game_mode::RemoteGameExecutionContext;
use serde_json::json;

// @api_command get_debug_real_physics_wireframe(): DebugCollector
pub fn get_debug_real_physics_wireframe(
    _payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let data = context.physics_system.debug_get_world();

    Ok(Some(json!(data).to_string()))
}
