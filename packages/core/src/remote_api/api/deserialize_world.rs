use ecs::component_trait::AttachedComponents;
use ecs::ecs_world::{ECSWorld, ECSWorldSerializedRepresentation};
use ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

pub fn deserialize_world(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: ECSWorldSerializedRepresentation = serde_json::from_str(payload).unwrap();
    ecs.clear();
    ecs.deserialize_into(input);

    json!({
        "success": true
    })
    .to_string()
}
