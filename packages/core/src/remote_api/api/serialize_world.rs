use ecs::ecs_world::ECSWorld;

// @api_command serialize_world(null): string
pub fn serialize_world(_payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    Ok(Some(serde_json::to_string(&ecs.serialize()).unwrap()))
}
