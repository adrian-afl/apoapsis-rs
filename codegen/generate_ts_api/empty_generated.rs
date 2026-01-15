use crate::remote_api::remote_game_mode::RemoteGameExecutionContext;

pub fn handle_message_api(
    name: &str,
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    Ok(None)
}
