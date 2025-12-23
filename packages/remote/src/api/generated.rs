
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
use ecs::component_trait::acquire_next_id;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetUniverseClockComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUniverseClockComponentInput {
    entity_id: String,
    component: UniverseClockComponent,
}

pub fn get_universe_clock(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUniverseClockComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.universe_clock).unwrap()
}

pub fn set_universe_clock(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUniverseClockComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.universe_clock = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_universe_clock(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUniverseClockComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.universe_clock = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetFirstPersonCameraControlComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetFirstPersonCameraControlComponentInput {
    entity_id: String,
    component: FirstPersonCameraControlComponent,
}

pub fn get_first_person_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetFirstPersonCameraControlComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.first_person_camera_control).unwrap()
}

pub fn set_first_person_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetFirstPersonCameraControlComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.first_person_camera_control = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_first_person_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetFirstPersonCameraControlComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.first_person_camera_control = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetThirdPersonOrbitCameraControlComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetThirdPersonOrbitCameraControlComponentInput {
    entity_id: String,
    component: ThirdPersonOrbitCameraControlComponent,
}

pub fn get_third_person_orbit_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetThirdPersonOrbitCameraControlComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.third_person_orbit_camera_control).unwrap()
}

pub fn set_third_person_orbit_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetThirdPersonOrbitCameraControlComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.third_person_orbit_camera_control = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_third_person_orbit_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetThirdPersonOrbitCameraControlComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.third_person_orbit_camera_control = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetThirdPersonStaticCameraControlComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetThirdPersonStaticCameraControlComponentInput {
    entity_id: String,
    component: ThirdPersonStaticCameraControlComponent,
}

pub fn get_third_person_static_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetThirdPersonStaticCameraControlComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.third_person_static_camera_control).unwrap()
}

pub fn set_third_person_static_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetThirdPersonStaticCameraControlComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.third_person_static_camera_control = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_third_person_static_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetThirdPersonStaticCameraControlComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.third_person_static_camera_control = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetTransformComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetTransformComponentInput {
    entity_id: String,
    component: TransformComponent,
}

pub fn get_transform(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetTransformComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.transform).unwrap()
}

pub fn set_transform(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetTransformComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.transform = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_transform(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetTransformComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.transform = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetRealPhysicsComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetRealPhysicsComponentInput {
    entity_id: String,
    component: RealPhysicsComponent,
}

pub fn get_real_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetRealPhysicsComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.real_physics).unwrap()
}

pub fn set_real_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetRealPhysicsComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.real_physics = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_real_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetRealPhysicsComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.real_physics = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetSimplePhysicsComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetSimplePhysicsComponentInput {
    entity_id: String,
    component: SimplePhysicsComponent,
}

pub fn get_simple_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetSimplePhysicsComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.simple_physics).unwrap()
}

pub fn set_simple_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetSimplePhysicsComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.simple_physics = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_simple_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetSimplePhysicsComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.simple_physics = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetShipControlComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetShipControlComponentInput {
    entity_id: String,
    component: ShipControlComponent,
}

pub fn get_ship_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetShipControlComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ship_control).unwrap()
}

pub fn set_ship_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetShipControlComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.ship_control = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ship_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetShipControlComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ship_control = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetUIColorComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIColorComponentInput {
    entity_id: String,
    component: UIColorComponent,
}

pub fn get_ui_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUIColorComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ui_color).unwrap()
}

pub fn set_ui_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUIColorComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.ui_color = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUIColorComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ui_color = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetUIHoverColorComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIHoverColorComponentInput {
    entity_id: String,
    component: UIHoverColorComponent,
}

pub fn get_ui_hover_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUIHoverColorComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ui_hover_color).unwrap()
}

pub fn set_ui_hover_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUIHoverColorComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.ui_hover_color = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_hover_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUIHoverColorComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ui_hover_color = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetUIBoxComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIBoxComponentInput {
    entity_id: String,
    component: UIBoxComponent,
}

pub fn get_ui_box(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUIBoxComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ui_box).unwrap()
}

pub fn set_ui_box(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUIBoxComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.ui_box = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_box(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUIBoxComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ui_box = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetUIHoverCursorComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIHoverCursorComponentInput {
    entity_id: String,
    component: UIHoverCursorComponent,
}

pub fn get_ui_hover_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUIHoverCursorComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ui_hover_cursor).unwrap()
}

pub fn set_ui_hover_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUIHoverCursorComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.ui_hover_cursor = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_hover_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUIHoverCursorComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ui_hover_cursor = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetUITextureComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUITextureComponentInput {
    entity_id: String,
    component: UITextureComponent,
}

pub fn get_ui_texture(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUITextureComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ui_texture).unwrap()
}

pub fn set_ui_texture(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUITextureComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.ui_texture = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_texture(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUITextureComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ui_texture = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetUITextComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUITextComponentInput {
    entity_id: String,
    component: UITextComponent,
}

pub fn get_ui_text(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUITextComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ui_text).unwrap()
}

pub fn set_ui_text(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUITextComponentInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id.as_str()].components.ui_text = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_text(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetUITextComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ui_text = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetSetPhysicsKinematicsComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddSetPhysicsKinematicsComponentInput {
    entity_id: String,
    component: SetPhysicsKinematicsComponent,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct RemoveSetPhysicsKinematicsComponentInput {
    entity_id: String,
    component_id: String,
}

pub fn get_set_physics_kinematics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetSetPhysicsKinematicsComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.set_physics_kinematics).unwrap()
}

pub fn add_set_physics_kinematics(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: AddSetPhysicsKinematicsComponentInput = serde_json::from_str(payload).unwrap();
    
    let new_id = acquire_next_id();
    input.component.id = new_id;

    ecs[input.entity_id.as_str()].components.set_physics_kinematics.push(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn remove_set_physics_kinematics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: RemoveSetPhysicsKinematicsComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.set_physics_kinematics.retain(|x| x.id != input.component_id.parse::<u64>().unwrap());

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct GetMeshComponentInput {
    entity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddMeshComponentInput {
    entity_id: String,
    component: MeshComponent,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct RemoveMeshComponentInput {
    entity_id: String,
    component_id: String,
}

pub fn get_mesh(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetMeshComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.mesh).unwrap()
}

pub fn add_mesh(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: AddMeshComponentInput = serde_json::from_str(payload).unwrap();
    
    let new_id = acquire_next_id();
    input.component.id = new_id;

    ecs[input.entity_id.as_str()].components.mesh.push(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn remove_mesh(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: RemoveMeshComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.mesh.retain(|x| x.id != input.component_id.parse::<u64>().unwrap());

    json!({
        "success": true
    })
    .to_string()
}


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
}

pub fn get_camera_focus(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.camera_focus).unwrap()
}

pub fn set_camera_focus(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.camera_focus = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_is_ground_collider(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.is_ground_collider).unwrap()
}

pub fn set_is_ground_collider(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.is_ground_collider = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_is_player(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.is_player).unwrap()
}

pub fn set_is_player(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.is_player = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_control_focus(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.control_focus).unwrap()
}

pub fn set_control_focus(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.control_focus = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_ui_is_raycastable(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ui_is_raycastable).unwrap()
}

pub fn set_ui_is_raycastable(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ui_is_raycastable = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_ui_require_free_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id.as_str()].components.ui_require_free_cursor).unwrap()
}

pub fn set_ui_require_free_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id.as_str()].components.ui_require_free_cursor = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn handle_message_components_api(
    name: &str,
    payload: &str,
    ecs: &mut ECSWorld,
) -> Result<String, String> {
    match name {
    "command.get_universe_clock" => Ok(get_universe_clock(payload, ecs)),
    "command.set_universe_clock" => Ok(set_universe_clock(payload, ecs)),
    "command.clear_universe_clock" => Ok(clear_universe_clock(payload, ecs)),
    "command.get_first_person_camera_control" => Ok(get_first_person_camera_control(payload, ecs)),
    "command.set_first_person_camera_control" => Ok(set_first_person_camera_control(payload, ecs)),
    "command.clear_first_person_camera_control" => Ok(clear_first_person_camera_control(payload, ecs)),
    "command.get_third_person_orbit_camera_control" => Ok(get_third_person_orbit_camera_control(payload, ecs)),
    "command.set_third_person_orbit_camera_control" => Ok(set_third_person_orbit_camera_control(payload, ecs)),
    "command.clear_third_person_orbit_camera_control" => Ok(clear_third_person_orbit_camera_control(payload, ecs)),
    "command.get_third_person_static_camera_control" => Ok(get_third_person_static_camera_control(payload, ecs)),
    "command.set_third_person_static_camera_control" => Ok(set_third_person_static_camera_control(payload, ecs)),
    "command.clear_third_person_static_camera_control" => Ok(clear_third_person_static_camera_control(payload, ecs)),
    "command.get_transform" => Ok(get_transform(payload, ecs)),
    "command.set_transform" => Ok(set_transform(payload, ecs)),
    "command.clear_transform" => Ok(clear_transform(payload, ecs)),
    "command.get_real_physics" => Ok(get_real_physics(payload, ecs)),
    "command.set_real_physics" => Ok(set_real_physics(payload, ecs)),
    "command.clear_real_physics" => Ok(clear_real_physics(payload, ecs)),
    "command.get_simple_physics" => Ok(get_simple_physics(payload, ecs)),
    "command.set_simple_physics" => Ok(set_simple_physics(payload, ecs)),
    "command.clear_simple_physics" => Ok(clear_simple_physics(payload, ecs)),
    "command.get_ship_control" => Ok(get_ship_control(payload, ecs)),
    "command.set_ship_control" => Ok(set_ship_control(payload, ecs)),
    "command.clear_ship_control" => Ok(clear_ship_control(payload, ecs)),
    "command.get_ui_color" => Ok(get_ui_color(payload, ecs)),
    "command.set_ui_color" => Ok(set_ui_color(payload, ecs)),
    "command.clear_ui_color" => Ok(clear_ui_color(payload, ecs)),
    "command.get_ui_hover_color" => Ok(get_ui_hover_color(payload, ecs)),
    "command.set_ui_hover_color" => Ok(set_ui_hover_color(payload, ecs)),
    "command.clear_ui_hover_color" => Ok(clear_ui_hover_color(payload, ecs)),
    "command.get_ui_box" => Ok(get_ui_box(payload, ecs)),
    "command.set_ui_box" => Ok(set_ui_box(payload, ecs)),
    "command.clear_ui_box" => Ok(clear_ui_box(payload, ecs)),
    "command.get_ui_hover_cursor" => Ok(get_ui_hover_cursor(payload, ecs)),
    "command.set_ui_hover_cursor" => Ok(set_ui_hover_cursor(payload, ecs)),
    "command.clear_ui_hover_cursor" => Ok(clear_ui_hover_cursor(payload, ecs)),
    "command.get_ui_texture" => Ok(get_ui_texture(payload, ecs)),
    "command.set_ui_texture" => Ok(set_ui_texture(payload, ecs)),
    "command.clear_ui_texture" => Ok(clear_ui_texture(payload, ecs)),
    "command.get_ui_text" => Ok(get_ui_text(payload, ecs)),
    "command.set_ui_text" => Ok(set_ui_text(payload, ecs)),
    "command.clear_ui_text" => Ok(clear_ui_text(payload, ecs)),
    "command.get_set_physics_kinematics" => Ok(get_set_physics_kinematics(payload, ecs)),
    "command.add_set_physics_kinematics" => Ok(add_set_physics_kinematics(payload, ecs)),
    "command.remove_set_physics_kinematics" => Ok(remove_set_physics_kinematics(payload, ecs)),
    "command.get_mesh" => Ok(get_mesh(payload, ecs)),
    "command.add_mesh" => Ok(add_mesh(payload, ecs)),
    "command.remove_mesh" => Ok(remove_mesh(payload, ecs)),
    "command.get_camera_focus" => Ok(get_camera_focus(payload, ecs)),
    "command.set_camera_focus" => Ok(set_camera_focus(payload, ecs)),
    "command.get_is_ground_collider" => Ok(get_is_ground_collider(payload, ecs)),
    "command.set_is_ground_collider" => Ok(set_is_ground_collider(payload, ecs)),
    "command.get_is_player" => Ok(get_is_player(payload, ecs)),
    "command.set_is_player" => Ok(set_is_player(payload, ecs)),
    "command.get_control_focus" => Ok(get_control_focus(payload, ecs)),
    "command.set_control_focus" => Ok(set_control_focus(payload, ecs)),
    "command.get_ui_is_raycastable" => Ok(get_ui_is_raycastable(payload, ecs)),
    "command.set_ui_is_raycastable" => Ok(set_ui_is_raycastable(payload, ecs)),
    "command.get_ui_require_free_cursor" => Ok(get_ui_require_free_cursor(payload, ecs)),
    "command.set_ui_require_free_cursor" => Ok(set_ui_require_free_cursor(payload, ecs)),
        _ => Err(format!("Handler not found for message {}", name)),
    }
}

