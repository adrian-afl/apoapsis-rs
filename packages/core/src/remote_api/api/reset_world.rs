use crate::remote_api::remote_game_mode::RemoteGameExecutionContext;
use ecs::ecs_world::ECSWorld;

// @api_command reset_world(): void
pub fn reset_world(
    _payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    context.ecs.clear();

    Ok(None)
}
