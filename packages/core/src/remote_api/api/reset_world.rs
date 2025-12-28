use ecs::component_trait::AttachedComponents;
use ecs::ecs_world::{ECSWorld, ECSWorldSerializedRepresentation};
use ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

pub fn reset_world(_payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    ecs.clear();

    Ok(None)
}
