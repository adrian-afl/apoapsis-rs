import * as fs from 'fs';

const singleComponentsRegex = /(.*?): ([A-Z][^<>]*?);$/gm;
const multiComponentsRegex = /(.*?): Array<(.*?)>;$/gm;
const booleanComponentsRegex = /(.*?): boolean;$/gm;

const attachedComponentsTypeStr = fs.readFileSync("../bindings/AttachedComponents.ts").toString("utf-8");

const singleComponentMatches = [...attachedComponentsTypeStr.matchAll(singleComponentsRegex)].map(x => ({
  snake: x[1].trim() as string,
  pascal: (x[2] as string).replace(/ \| null$/, '')
}));
const multiComponentsMatches = [...attachedComponentsTypeStr.matchAll(multiComponentsRegex)].map(x => ({
  snake: x[1].trim() as string,
  pascal: x[2] as string
}));
const booleanComponentsMatches = [...attachedComponentsTypeStr.matchAll(booleanComponentsRegex)].map(x => x[1].trim() as string);

// console.log({
//   singleComponentMatches,
//   multiComponentsMatches,
//   booleanComponentsMatches
// });

const preamble = `
use ecs::components::camera::camera_focus_component::CameraFocusComponent;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::camera::third_person_orbit_camera_control_component::ThirdPersonOrbitCameraControlComponent;
use ecs::components::camera::third_person_static_camera_control_component::ThirdPersonStaticCameraControlComponent;
use ecs::components::common::control_focus_component::ControlFocusComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::components::physics::is_ground_collider_component::IsGroundColliderComponent;
use ecs::components::physics::real_physics_component::RealPhysicsComponent;
use ecs::components::physics::set_physics_kinematics_component::SetPhysicsKinematicsComponent;
use ecs::components::physics::simple_physics_component::SimplePhysicsComponent;
use ecs::components::player::is_player_component::IsPlayerComponent;
use ecs::components::rendering::mesh_component::MeshComponent;
use ecs::components::ship::ship_control_component::ShipControlComponent;
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_color_component::UIColorComponent;
use ecs::components::ui::ui_hover_color_component::UIHoverColorComponent;
use ecs::components::ui::ui_hover_cursor_component::UIHoverCursorComponent;
use ecs::components::ui::ui_is_raycastable_component::UIIsRaycastableComponent;
use ecs::components::ui::ui_require_free_cursor_component::UIRequireFreeCursorComponent;
use ecs::components::ui::ui_text_component::UITextComponent;
use ecs::components::ui::ui_texture_component::UITextureComponent;
use ecs::ecs_world::ECSWorld;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use ecs::component_trait::acquire_next_id;`;

console.log(preamble);

for (const component of singleComponentMatches) {
  console.log(`
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct Get${component.pascal}Input {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct Set${component.pascal}Input {
    entity_id: String,
    component: ${component.pascal},
}

pub fn get_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: Get${component.pascal}Input = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.${component.snake}).unwrap()
}

pub fn set_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: Set${component.pascal}Input = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.${component.snake} = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: Get${component.pascal}Input = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.${component.snake} = None;

    json!({
        "success": true
    })
    .to_string()
}
`);
}


for (const component of multiComponentsMatches) {
  console.log(`
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct Get${component.pascal}Input {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct Add${component.pascal}Input {
    entity_id: String,
    component: ${component.pascal},
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct Remove${component.pascal}Input {
    entity_id: String,
    component_id: String,
}

pub fn get_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: Get${component.pascal}Input = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.${component.snake}).unwrap()
}

pub fn add_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: Add${component.pascal}Input = serde_json::from_str(payload).unwrap();
    
    let new_id = acquire_next_id();
    input.component.id = new_id;

    ecs[input.entity_id.as_str()].components.${component.snake}.push(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn remove_${component.snake}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: Remove${component.pascal}Input = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.${component.snake}.retain(|x| x.id != input.component_id.parse::<u64>().unwrap());

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
struct GetBooleanComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetBooleanComponentInput {
    entity_id: String,
    value: bool,
}`);

for (const component of booleanComponentsMatches) {
  console.log(`
pub fn get_${component}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.${component}).unwrap()
}

pub fn set_${component}(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.${component} = input.value;

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
    ${singleComponentMatches.map(x =>
  `"command.get_${x.snake}" => Ok(get_${x.snake}(payload, ecs)),
    "command.set_${x.snake}" => Ok(set_${x.snake}(payload, ecs)),
    "command.clear_${x.snake}" => Ok(clear_${x.snake}(payload, ecs)),`).join("\n    ")}
    ${multiComponentsMatches.map(x =>
  `"command.get_${x.snake}" => Ok(get_${x.snake}(payload, ecs)),
    "command.add_${x.snake}" => Ok(add_${x.snake}(payload, ecs)),
    "command.remove_${x.snake}" => Ok(remove_${x.snake}(payload, ecs)),`).join("\n    ")}
    ${booleanComponentsMatches.map(x =>
  `"command.get_${x}" => Ok(get_${x}(payload, ecs)),
    "command.set_${x}" => Ok(set_${x}(payload, ecs)),`).join("\n    ")}
        _ => Err(format!("Handler not found for message {}", name)),
    }
}
`)