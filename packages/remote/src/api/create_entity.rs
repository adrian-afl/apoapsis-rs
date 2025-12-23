use ecs::component_trait::AttachedComponents;
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct CreateEntityInput {
    name: Option<String>,
    components: AttachedComponents,
}

pub fn create_entity(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: CreateEntityInput = serde_json::from_str(payload).unwrap();
    let entity = Entity::new(input.name.as_deref());
    let id = entity.id;
    ecs.add(entity);

    json!({
        "success": true,
        "id": id.to_string()
    })
    .to_string()
}
