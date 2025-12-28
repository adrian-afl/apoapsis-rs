use ecs::ecs_world::{ECSWorld, ECSWorldSerializedRepresentation};

// @api_command deserialize_world(ECSWorldSerializedRepresentation): void
pub fn deserialize_world(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: ECSWorldSerializedRepresentation = serde_json::from_str(payload).unwrap();
    ecs.clear();
    ecs.deserialize_into(input);

    Ok(None)
}
