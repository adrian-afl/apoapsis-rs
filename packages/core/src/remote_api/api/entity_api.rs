use crate::remote_api::util::serde_parse_err_map;
use ecs::component_trait::AttachedComponents;
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddEntityInput {
    name: Option<String>,
    components: Option<AttachedComponents>,
}

// @api_command add_entity(AddEntityInput): ObjectWithID
pub fn add_entity(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: AddEntityInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;
    let entity = Entity::new(input.name.as_deref());
    let id = entity.id;
    ecs.add(entity);

    Ok(Some(
        json!({
            "id": id
        })
        .to_string(),
    ))
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct RemoveEntityInput {
    name: Option<String>,
    components: Option<AttachedComponents>,
}

// @api_command remove_entity(number): void
pub fn remove_entity(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let id: u64 = payload.parse().map_err(|_| "Invalid id")?;
    ecs.remove_by_id(id);

    Ok(None)
}
