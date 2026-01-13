use crate::remote_api::remote_game_mode::RemoteGameExecutionContext;
use crate::remote_api::util::serde_parse_err_map;
use ecs::component_trait::{AttachedComponents, Components};
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddEntityInput {
    components: Option<AttachedComponents>,
}

// @api_command add_entity(input: AddEntityInput): ObjectWithID
pub fn add_entity(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: AddEntityInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;
    let entity = match input.components {
        None => Entity::new(),
        Some(components) => Entity::new_with_components(components),
    };
    let id = entity.id;
    context.ecs.add(entity);

    Ok(Some(
        json!({
            "id": id
        })
        .to_string(),
    ))
}

// @api_command remove_entity(id: number): void
pub fn remove_entity(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let id: u64 = payload.parse().map_err(|_| "Invalid id")?;
    context.ecs.remove_by_id(id);

    Ok(None)
}

// @api_command get_entity(id: number): Entity
pub fn get_entity(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let id: u64 = payload.parse().map_err(|_| "Invalid id")?;

    Ok(Some(
        serde_json::to_string(&context.ecs.find_by_id(id).ok_or("Entity not found")?)
            .map_err(|_| "Cannot serialize")?,
    ))
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct ReplaceEntityComponentsInput {
    id: u64,
    components: AttachedComponents,
}

// @api_command replace_entity_components(id: number, components: AttachedComponents): void
pub fn replace_entity_components(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: ReplaceEntityComponentsInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let entity = context
        .ecs
        .find_by_id_mut(input.id)
        .ok_or("Entity not found")?;
    entity.components = input.components;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct FindAllEntitiesByComponents {
    components: Vec<Components>,
}

// @api_command find_all_entities_by_components(components: AttachedComponents[]): number[]
pub fn find_all_entities_by_components(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: FindAllEntitiesByComponents =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let components: Vec<&Components> = input.components.iter().map(|x| x).collect();

    Ok(Some(
        serde_json::to_string(&context.ecs.find_all_ids_by_components(&*components))
            .map_err(|_| "Cannot serialize")?,
    ))
}
