use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::components::camera::camera_focus_component::CameraFocusComponent;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::camera::third_person_orbit_camera_control_component::ThirdPersonOrbitCameraControlComponent;
use ecs::components::camera::third_person_static_camera_control_component::ThirdPersonStaticCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::physics::is_ground_collider_component::IsGroundColliderComponent;
use ecs::components::physics::real_physics_component::RealPhysicsComponent;
use ecs::components::physics::simple_physics_component::SimplePhysicsComponent;
use ecs::components::physics::set_physics_kinematics_component::SetPhysicsKinematicsComponent;
use ecs::components::player::is_player_component::IsPlayerComponent;
use ecs::components::rendering::mesh_component::MeshComponent;
use ecs::components::common::control_focus_component::ControlFocusComponent;
use ecs::components::ship::ship_control_component::ShipControlComponent;
use ecs::components::ui::ui_color_component::UIColorComponent;
use ecs::components::ui::ui_hover_color_component::UIHoverColorComponent;
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_hover_cursor_component::UIHoverCursorComponent;
use ecs::components::ui::ui_texture_component::UITextureComponent;
use ecs::components::ui::ui_text_component::UITextComponent;
use ecs::components::ui::ui_is_raycastable_component::UIIsRaycastableComponent;
use ecs::components::ui::ui_require_free_cursor_component::UIRequireFreeCursorComponent;
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



#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUniverseClockInput {
    entity_id: u64,
    component: UniverseClockComponent,
}

pub fn get_universe_clock(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.universe_clock).unwrap()
}

pub fn set_universe_clock(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUniverseClockInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.universe_clock = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_universe_clock(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.universe_clock = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetFirstPersonCameraControlInput {
    entity_id: u64,
    component: FirstPersonCameraControlComponent,
}

pub fn get_first_person_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.first_person_camera_control).unwrap()
}

pub fn set_first_person_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetFirstPersonCameraControlInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.first_person_camera_control = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_first_person_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.first_person_camera_control = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetThirdPersonOrbitCameraControlInput {
    entity_id: u64,
    component: ThirdPersonOrbitCameraControlComponent,
}

pub fn get_third_person_orbit_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.third_person_orbit_camera_control).unwrap()
}

pub fn set_third_person_orbit_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetThirdPersonOrbitCameraControlInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.third_person_orbit_camera_control = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_third_person_orbit_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.third_person_orbit_camera_control = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetThirdPersonStaticCameraControlInput {
    entity_id: u64,
    component: ThirdPersonStaticCameraControlComponent,
}

pub fn get_third_person_static_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.third_person_static_camera_control).unwrap()
}

pub fn set_third_person_static_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetThirdPersonStaticCameraControlInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.third_person_static_camera_control = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_third_person_static_camera_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.third_person_static_camera_control = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetTransformInput {
    entity_id: u64,
    component: TransformComponent,
}

pub fn get_transform(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.transform).unwrap()
}

pub fn set_transform(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetTransformInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.transform = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_transform(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.transform = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetRealPhysicsInput {
    entity_id: u64,
    component: RealPhysicsComponent,
}

pub fn get_real_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.real_physics).unwrap()
}

pub fn set_real_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetRealPhysicsInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.real_physics = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_real_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.real_physics = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetSimplePhysicsInput {
    entity_id: u64,
    component: SimplePhysicsComponent,
}

pub fn get_simple_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.simple_physics).unwrap()
}

pub fn set_simple_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetSimplePhysicsInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.simple_physics = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_simple_physics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.simple_physics = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetShipControlInput {
    entity_id: u64,
    component: ShipControlComponent,
}

pub fn get_ship_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ship_control).unwrap()
}

pub fn set_ship_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetShipControlInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.ship_control = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ship_control(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ship_control = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIColorInput {
    entity_id: u64,
    component: UIColorComponent,
}

pub fn get_ui_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ui_color).unwrap()
}

pub fn set_ui_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUIColorInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.ui_color = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ui_color = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIHoverColorInput {
    entity_id: u64,
    component: UIHoverColorComponent,
}

pub fn get_ui_hover_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ui_hover_color).unwrap()
}

pub fn set_ui_hover_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUIHoverColorInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.ui_hover_color = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_hover_color(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ui_hover_color = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIBoxInput {
    entity_id: u64,
    component: UIBoxComponent,
}

pub fn get_ui_box(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ui_box).unwrap()
}

pub fn set_ui_box(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUIBoxInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.ui_box = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_box(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ui_box = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIHoverCursorInput {
    entity_id: u64,
    component: UIHoverCursorComponent,
}

pub fn get_ui_hover_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ui_hover_cursor).unwrap()
}

pub fn set_ui_hover_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUIHoverCursorInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.ui_hover_cursor = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_hover_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ui_hover_cursor = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUITextureInput {
    entity_id: u64,
    component: UITextureComponent,
}

pub fn get_ui_texture(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ui_texture).unwrap()
}

pub fn set_ui_texture(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUITextureInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.ui_texture = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_texture(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ui_texture = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUITextInput {
    entity_id: u64,
    component: UITextComponent,
}

pub fn get_ui_text(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ui_text).unwrap()
}

pub fn set_ui_text(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: SetUITextInput = serde_json::from_str(payload).unwrap();

    let new_id = acquire_next_id();
    input.component.id = new_id;
    
    ecs[input.entity_id].components.ui_text = Some(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn clear_ui_text(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ui_text = None;

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddSetPhysicsKinematicsInput {
    entity_id: u64,
    component: SetPhysicsKinematicsComponent,
}

pub fn get_set_physics_kinematics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.set_physics_kinematics).unwrap()
}

pub fn add_set_physics_kinematics(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: AddSetPhysicsKinematicsInput = serde_json::from_str(payload).unwrap();
    
    let new_id = acquire_next_id();
    input.component.id = new_id;

    ecs[input.entity_id].components.set_physics_kinematics.push(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn remove_set_physics_kinematics(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: RemoveComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.set_physics_kinematics.retain(|x| x.id != input.component_id.parse::<u64>().unwrap());

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddMeshInput {
    entity_id: u64,
    component: MeshComponent,
}

pub fn get_mesh(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.mesh).unwrap()
}

pub fn add_mesh(payload: &str, ecs: &mut ECSWorld) -> String {
    let mut input: AddMeshInput = serde_json::from_str(payload).unwrap();
    
    let new_id = acquire_next_id();
    input.component.id = new_id;

    ecs[input.entity_id].components.mesh.push(input.component);

    json!({
        "success": true,
        "id": new_id,
    })
    .to_string()
}

pub fn remove_mesh(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: RemoveComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.mesh.retain(|x| x.id != input.component_id.parse::<u64>().unwrap());

    json!({
        "success": true
    })
    .to_string()
}


#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetBooleanComponentInput {
    entity_id: u64,
    value: bool,
}

pub fn get_camera_focus(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.camera_focus).unwrap()
}

pub fn set_camera_focus(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.camera_focus = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_is_ground_collider(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.is_ground_collider).unwrap()
}

pub fn set_is_ground_collider(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.is_ground_collider = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_is_player(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.is_player).unwrap()
}

pub fn set_is_player(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.is_player = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_control_focus(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.control_focus).unwrap()
}

pub fn set_control_focus(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.control_focus = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_ui_is_raycastable(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ui_is_raycastable).unwrap()
}

pub fn set_ui_is_raycastable(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ui_is_raycastable = input.value;

    json!({
        "success": true,
    })
    .to_string()
}



pub fn get_ui_require_free_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: GetComponentInput = serde_json::from_str(payload).unwrap();
    
    serde_json::to_string(&ecs[input.entity_id].components.ui_require_free_cursor).unwrap()
}

pub fn set_ui_require_free_cursor(payload: &str, ecs: &mut ECSWorld) -> String {
    let input: SetBooleanComponentInput = serde_json::from_str(payload).unwrap();
    
    ecs[input.entity_id].components.ui_require_free_cursor = input.value;

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

