import { readComponentsMetadata } from "./readComponentsMetadata.js";

const componentsMetadata = readComponentsMetadata();

console.log(`${componentsMetadata.map((x) => x.importRs).join("\n")}
use ecs::ecs_world::ECSWorld;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use ecs::component_trait::acquire_next_id;


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
    component_id: String,
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

pub fn get_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.${component.snake}).unwrap()
}

pub fn set_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: Set${component.short}Input = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.${component.snake} = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.${component.snake} = None;

    json!({
        "success": true
    })
    .to_string()
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

pub fn get_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.${component.snake}).unwrap()
}

pub fn add_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: Add${component.short}Input = serde_json::from_str(payload).unwrap();
    
    let new_id = acquire_next_id();
    input.component.id = new_id;

    ecs[input.entity_id].components.${component.snake}.push(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn remove_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: RemoveComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.${component.snake}.retain(|x| x.id != input.component_id.parse::<u64>().unwrap());

    json!({
        "success": true
    })
    .to_string()
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
pub fn get_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.${component.snake}).unwrap()
}

pub fn set_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.${component.snake} = input.value;

    json!({
        "success": true,
    })
    .to_string()
}

`);
}

console.log(`
pub fn handle_message_components_api(
    name: &str,
    payload: &str,
    ecs: &mut ECSWorld,
) -> Result<String, String> {
    match name {
    ${componentsMetadata
      .filter((x) => x.type === "Option")
      .map(
        (x) =>
          `"command.get_${x.snake}" => Ok(get_${x.snake}(payload, ecs)),
    "command.set_${x.snake}" => Ok(set_${x.snake}(payload, ecs)),
    "command.clear_${x.snake}" => Ok(clear_${x.snake}(payload, ecs)),`,
      )
      .join("\n    ")}
    ${componentsMetadata
      .filter((x) => x.type === "Vector")
      .map(
        (x) =>
          `"command.get_${x.snake}" => Ok(get_${x.snake}(payload, ecs)),
    "command.add_${x.snake}" => Ok(add_${x.snake}(payload, ecs)),
    "command.remove_${x.snake}" => Ok(remove_${x.snake}(payload, ecs)),`,
      )
      .join("\n    ")}
    ${componentsMetadata
      .filter((x) => x.type === "Marker")
      .map(
        (x) =>
          `"command.get_${x.snake}" => Ok(get_${x.snake}(payload, ecs)),
    "command.set_${x.snake}" => Ok(set_${x.snake}(payload, ecs)),`,
      )
      .join("\n    ")}
        _ => Err(format!("Handler not found for message {}", name)),
    }
}
`);
