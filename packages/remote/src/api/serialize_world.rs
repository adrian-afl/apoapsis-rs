use ecs::ecs_world::ECSWorld;
use serde::Serialize;

pub fn serialize_world(_payload: &str, ecs: &mut ECSWorld) -> String {
    serde_json::to_string(&ecs.serialize()).unwrap()
}
