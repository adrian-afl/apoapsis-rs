use crate::remote_api::util::serde_parse_err_map;
use ecs::ecs_world::{ECSWorld, ECSWorldSerializedRepresentation};

// @api_command deserialize_world(serializedWorld: ECSWorldSerializedRepresentation): void
pub fn deserialize_world(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: ECSWorldSerializedRepresentation =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;
    ecs.clear();
    ecs.deserialize_into(input);

    Ok(None)
}
