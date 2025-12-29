import { readComponentsMetadata } from "./readComponentsMetadata.js";
import { readApiExports } from "./readApiExports.js";

const componentsMetadata = readComponentsMetadata();
const apiExports = readApiExports();

console.log(`${componentsMetadata.map((x) => x.importRs).join("\n")}
${apiExports.commands.map((x) => x.importRs).join("\n")}
use ecs::ecs_world::ECSWorld;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use ecs::component_trait::acquire_next_id;
use crate::remote_api::util::serde_err_map;


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetComponentInput {
    entity_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct RemoveComponentInput {
    entity_id: u64,
    component_id: u64,
}

`);

for (const component of componentsMetadata.filter((x) => x.type === "Option")) {
  console.log(`
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct Set${component.short}Input {
    entity_id: u64,
    component: ${component.full},
}

pub fn get_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_err_map)?;
    
    Ok(Some(serde_json::to_string(
        &ecs.find_by_id(input.entity_id).ok_or("Entity not found")?.components.${component.snake}).map_err(|_| "Cannot serialize")?
    ))
}

pub fn set_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let mut input: Set${component.short}Input = serde_json::from_str(payload).map_err(serde_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs.find_by_id_mut(input.entity_id).ok_or("Entity not found")?.components.${component.snake} = Some(input.component);

    Ok(Some(json!({
        "id": new_id,
    })
    .to_string()))
}

pub fn clear_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_err_map)?;
    
    ecs.find_by_id_mut(input.entity_id).ok_or("Entity not found")?.components.${component.snake} = None;

    Ok(None)
}
`);
}

for (const component of componentsMetadata.filter((x) => x.type === "Vector")) {
  console.log(`
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct Add${component.short}Input {
    entity_id: u64,
    component: ${component.full},
}

pub fn get_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_err_map)?;
    
    Ok(Some(serde_json::to_string(&ecs.find_by_id(input.entity_id).ok_or("Entity not found")?.components.${component.snake}).map_err(|_| "Cannot serialize")?))
}

pub fn add_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let mut input: Add${component.short}Input = serde_json::from_str(payload).map_err(serde_err_map)?;
    
    let new_id = acquire_next_id();
    input.component.id = new_id;

    ecs.find_by_id_mut(input.entity_id).ok_or("Entity not found")?.components.${component.snake}.push(input.component);

    Ok(Some(json!({
        "id": new_id,
    })
    .to_string()))
}

pub fn remove_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: RemoveComponentInput = serde_json::from_str(payload).map_err(serde_err_map)?;
    
    ecs.find_by_id_mut(input.entity_id).ok_or("Entity not found")?.components.${component.snake}.retain(|x| x.id != input.component_id);

    Ok(None)
}
`);
}
console.log(`
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetBooleanComponentInput {
    entity_id: u64,
    value: bool,
}`);

for (const component of componentsMetadata.filter((x) => x.type === "Marker")) {
  console.log(`
pub fn get_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_err_map)?;
    
    Ok(Some(serde_json::to_string(&ecs.find_by_id(input.entity_id).ok_or("Entity not found")?.components.${component.snake}).map_err(|_| "Cannot serialize")?))
}

pub fn set_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).map_err(serde_err_map)?;
    
    ecs.find_by_id_mut(input.entity_id).ok_or("Entity not found")?.components.${component.snake} = input.value;

    Ok(None)
}

`);
}

console.log(`
pub fn handle_message_api(
    name: &str,
    payload: &str,
    ecs: &mut ECSWorld,
) -> Result<Option<String>, String> {
    match name {
    ${apiExports.commands
      .map((x) => `"command.${x.name}" => ${x.name}(payload, ecs),`)
      .join("\n    ")}
    ${componentsMetadata
      .filter((x) => x.type === "Option")
      .map(
        (x) =>
          `"command.get_${x.snake}" => get_${x.snake}(payload, ecs),
    "command.set_${x.snake}" => set_${x.snake}(payload, ecs),
    "command.clear_${x.snake}" => clear_${x.snake}(payload, ecs),`,
      )
      .join("\n    ")}
    ${componentsMetadata
      .filter((x) => x.type === "Vector")
      .map(
        (x) =>
          `"command.get_${x.snake}" => get_${x.snake}(payload, ecs),
    "command.add_${x.snake}" => add_${x.snake}(payload, ecs),
    "command.remove_${x.snake}" => remove_${x.snake}(payload, ecs),`,
      )
      .join("\n    ")}
    ${componentsMetadata
      .filter((x) => x.type === "Marker")
      .map(
        (x) =>
          `"command.get_${x.snake}" => get_${x.snake}(payload, ecs),
    "command.set_${x.snake}" => set_${x.snake}(payload, ecs),`,
      )
      .join("\n    ")}
        _ => Err(format!("Handler not found for message {}", name)),
    }
}
`);
