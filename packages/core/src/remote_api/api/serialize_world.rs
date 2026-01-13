use crate::remote_api::remote_game_mode::RemoteGameExecutionContext;
use crate::remote_api::util::serde_serialize_err_map;
use ecs::ecs_world::ECSWorld;

// @api_command serialize_world(): ECSWorldSerializedRepresentation
pub fn serialize_world(
    _payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    Ok(Some(
        serde_json::to_string(&context.ecs.serialize()).map_err(serde_serialize_err_map)?,
    ))
}
