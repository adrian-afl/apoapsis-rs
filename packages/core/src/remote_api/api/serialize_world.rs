use crate::remote_api::util::serde_serialize_err_map;
use ecs::ecs_world::ECSWorld;

// @api_command serialize_world(): ECSWorldSerializedRepresentation
pub fn serialize_world(_payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    Ok(Some(
        serde_json::to_string(&ecs.serialize()).map_err(serde_serialize_err_map)?,
    ))
}
