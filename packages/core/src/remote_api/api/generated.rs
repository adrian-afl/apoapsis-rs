use crate::remote_api::api::deserialize_world::deserialize_world;
use crate::remote_api::api::entity_api::add_entity;
use crate::remote_api::api::entity_api::find_all_entities_by_components;
use crate::remote_api::api::entity_api::get_entity;
use crate::remote_api::api::entity_api::remove_entity;
use crate::remote_api::api::entity_api::replace_entity_components;
use crate::remote_api::api::physics_api::raycast_real_physics;
use crate::remote_api::api::reset_world::reset_world;
use crate::remote_api::api::serialize_world::serialize_world;
use crate::remote_api::api::universe_simulation_api::get_all_celestial_body_names;
use crate::remote_api::api::universe_simulation_api::get_approximate_altitude_over_celestial_body;
use crate::remote_api::api::universe_simulation_api::get_celestial_body_definition;
use crate::remote_api::api::universe_simulation_api::get_celestial_body_orientation;
use crate::remote_api::api::universe_simulation_api::get_celestial_body_parent;
use crate::remote_api::api::universe_simulation_api::get_celestial_body_position;
use crate::remote_api::api::universe_simulation_api::get_celestial_body_satellites;
use crate::remote_api::api::universe_simulation_api::get_celestial_body_surface_velocity;
use crate::remote_api::api::universe_simulation_api::get_closest_celestial_body;
use crate::remote_api::api::universe_simulation_api::get_gravity_flux;
use crate::remote_api::api::universe_simulation_api::get_real_altitude_over_celestial_body;
use crate::remote_api::remote_game_mode::RemoteGameExecutionContext;
use crate::remote_api::util::{serde_parse_err_map, serde_serialize_err_map};
use ecs::component_trait::acquire_next_id;
use ecs::components::camera::first_person_camera_control_component::FirstPersonCameraControlComponent;
use ecs::components::camera::third_person_orbit_camera_control_component::ThirdPersonOrbitCameraControlComponent;
use ecs::components::camera::third_person_static_camera_control_component::ThirdPersonStaticCameraControlComponent;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::common::universe_clock_component::UniverseClockComponent;
use ecs::components::physics::glue_to_celestial_body_component::GlueToCelestialBodyComponent;
use ecs::components::physics::real_physics_component::RealPhysicsComponent;
use ecs::components::physics::set_physics_kinematics_component::SetPhysicsKinematicsComponent;
use ecs::components::physics::simple_physics_component::SimplePhysicsComponent;
use ecs::components::rendering::mesh_component::MeshComponent;
use ecs::components::ship::ship_control_component::ShipControlComponent;
use ecs::components::ui::ui_box_component::UIBoxComponent;
use ecs::components::ui::ui_color_component::UIColorComponent;
use ecs::components::ui::ui_hover_color_component::UIHoverColorComponent;
use ecs::components::ui::ui_hover_cursor_component::UIHoverCursorComponent;
use ecs::components::ui::ui_text_component::UITextComponent;
use ecs::components::ui::ui_texture_component::UITextureComponent;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

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

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUniverseClockInput {
    entity_id: u64,
    component: UniverseClockComponent,
}

pub fn get_universe_clock(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .universe_clock,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_universe_clock(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetUniverseClockInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .universe_clock = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_universe_clock(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .universe_clock = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetFirstPersonCameraControlInput {
    entity_id: u64,
    component: FirstPersonCameraControlComponent,
}

pub fn get_first_person_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .first_person_camera_control,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_first_person_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetFirstPersonCameraControlInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .first_person_camera_control = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_first_person_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .first_person_camera_control = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetThirdPersonOrbitCameraControlInput {
    entity_id: u64,
    component: ThirdPersonOrbitCameraControlComponent,
}

pub fn get_third_person_orbit_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .third_person_orbit_camera_control,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_third_person_orbit_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetThirdPersonOrbitCameraControlInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .third_person_orbit_camera_control = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_third_person_orbit_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .third_person_orbit_camera_control = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetThirdPersonStaticCameraControlInput {
    entity_id: u64,
    component: ThirdPersonStaticCameraControlComponent,
}

pub fn get_third_person_static_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .third_person_static_camera_control,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_third_person_static_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetThirdPersonStaticCameraControlInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .third_person_static_camera_control = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_third_person_static_camera_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .third_person_static_camera_control = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetTransformInput {
    entity_id: u64,
    component: TransformComponent,
}

pub fn get_transform(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .transform,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_transform(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetTransformInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .transform = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_transform(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .transform = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetRealPhysicsInput {
    entity_id: u64,
    component: RealPhysicsComponent,
}

pub fn get_real_physics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .real_physics,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_real_physics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetRealPhysicsInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .real_physics = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_real_physics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .real_physics = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetSimplePhysicsInput {
    entity_id: u64,
    component: SimplePhysicsComponent,
}

pub fn get_simple_physics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .simple_physics,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_simple_physics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetSimplePhysicsInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .simple_physics = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_simple_physics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .simple_physics = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetGlueToCelestialBodyInput {
    entity_id: u64,
    component: GlueToCelestialBodyComponent,
}

pub fn get_glue_to_celestial_body(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .glue_to_celestial_body,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_glue_to_celestial_body(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetGlueToCelestialBodyInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .glue_to_celestial_body = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_glue_to_celestial_body(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .glue_to_celestial_body = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetShipControlInput {
    entity_id: u64,
    component: ShipControlComponent,
}

pub fn get_ship_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ship_control,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ship_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetShipControlInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ship_control = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_ship_control(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ship_control = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIColorInput {
    entity_id: u64,
    component: UIColorComponent,
}

pub fn get_ui_color(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ui_color,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ui_color(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetUIColorInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_color = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_ui_color(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_color = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIHoverColorInput {
    entity_id: u64,
    component: UIHoverColorComponent,
}

pub fn get_ui_hover_color(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ui_hover_color,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ui_hover_color(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetUIHoverColorInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_hover_color = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_ui_hover_color(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_hover_color = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIBoxInput {
    entity_id: u64,
    component: UIBoxComponent,
}

pub fn get_ui_box(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ui_box,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ui_box(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetUIBoxInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_box = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_ui_box(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_box = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUIHoverCursorInput {
    entity_id: u64,
    component: UIHoverCursorComponent,
}

pub fn get_ui_hover_cursor(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ui_hover_cursor,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ui_hover_cursor(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetUIHoverCursorInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_hover_cursor = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_ui_hover_cursor(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_hover_cursor = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUITextureInput {
    entity_id: u64,
    component: UITextureComponent,
}

pub fn get_ui_texture(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ui_texture,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ui_texture(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetUITextureInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_texture = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_ui_texture(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_texture = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetUITextInput {
    entity_id: u64,
    component: UITextComponent,
}

pub fn get_ui_text(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ui_text,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ui_text(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: SetUITextInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_text = Some(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn clear_ui_text(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_text = None;

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddSetPhysicsKinematicsInput {
    entity_id: u64,
    component: SetPhysicsKinematicsComponent,
}

pub fn get_set_physics_kinematics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .set_physics_kinematics,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn add_set_physics_kinematics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: AddSetPhysicsKinematicsInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .set_physics_kinematics
        .push(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn remove_set_physics_kinematics(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: RemoveComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .set_physics_kinematics
        .retain(|x| x.id != input.component_id);

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct AddMeshInput {
    entity_id: u64,
    component: MeshComponent,
}

pub fn get_mesh(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .mesh,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn add_mesh(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let mut input: AddMeshInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    let new_id = acquire_next_id();
    input.component.id = new_id;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .mesh
        .push(input.component);

    Ok(Some(
        json!({
            "id": new_id,
        })
        .to_string(),
    ))
}

pub fn remove_mesh(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: RemoveComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .mesh
        .retain(|x| x.id != input.component_id);

    Ok(None)
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct SetBooleanComponentInput {
    entity_id: u64,
    value: bool,
}

pub fn get_camera_focus(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .camera_focus,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_camera_focus(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: SetBooleanComponentInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .camera_focus = input.value;

    Ok(None)
}

pub fn get_is_ground_collider(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .is_ground_collider,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_is_ground_collider(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: SetBooleanComponentInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .is_ground_collider = input.value;

    Ok(None)
}

pub fn get_is_celestial_body_surface(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .is_celestial_body_surface,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_is_celestial_body_surface(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: SetBooleanComponentInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    // context.ecs.find_by_id_mut(input.entity_id).ok_or("Entity not found")?.components.is_celestial_body_surface = input.value;

    Ok(None)
}

pub fn get_is_player(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .is_player,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_is_player(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: SetBooleanComponentInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .is_player = input.value;

    Ok(None)
}

pub fn get_control_focus(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .control_focus,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_control_focus(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: SetBooleanComponentInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .control_focus = input.value;

    Ok(None)
}

pub fn get_ui_is_raycastable(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ui_is_raycastable,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ui_is_raycastable(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: SetBooleanComponentInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_is_raycastable = input.value;

    Ok(None)
}

pub fn get_ui_require_free_cursor(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: GetComponentInput = serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    Ok(Some(
        serde_json::to_string(
            &context
                .ecs
                .find_by_id(input.entity_id)
                .ok_or("Entity not found")?
                .components
                .ui_require_free_cursor,
        )
        .map_err(serde_serialize_err_map)?,
    ))
}

pub fn set_ui_require_free_cursor(
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    let input: SetBooleanComponentInput =
        serde_json::from_str(payload).map_err(serde_parse_err_map)?;

    context
        .ecs
        .find_by_id_mut(input.entity_id)
        .ok_or("Entity not found")?
        .components
        .ui_require_free_cursor = input.value;

    Ok(None)
}

pub fn handle_message_api(
    name: &str,
    payload: &str,
    context: &mut RemoteGameExecutionContext,
) -> Result<Option<String>, String> {
    match name {
        "command.deserialize_world" => deserialize_world(payload, context),
        "command.add_entity" => add_entity(payload, context),
        "command.remove_entity" => remove_entity(payload, context),
        "command.get_entity" => get_entity(payload, context),
        "command.replace_entity_components" => replace_entity_components(payload, context),
        "command.find_all_entities_by_components" => {
            find_all_entities_by_components(payload, context)
        }
        "command.get_debug_real_physics_wireframe" => {
            find_all_entities_by_components(payload, context)
            // get_debug_real_physics_wireframe(payload, context)
        }
        "command.raycast_real_physics" => raycast_real_physics(payload, context),
        "command.reset_world" => reset_world(payload, context),
        "command.serialize_world" => serialize_world(payload, context),
        "command.get_all_celestial_body_names" => get_all_celestial_body_names(payload, context),
        "command.get_celestial_body_position" => get_celestial_body_position(payload, context),
        "command.get_celestial_body_definition" => get_celestial_body_definition(payload, context),
        "command.get_celestial_body_surface_velocity" => {
            get_celestial_body_surface_velocity(payload, context)
        }
        "command.get_celestial_body_orientation" => {
            get_celestial_body_orientation(payload, context)
        }
        "command.get_celestial_body_parent" => get_celestial_body_parent(payload, context),
        "command.get_celestial_body_satellites" => get_celestial_body_satellites(payload, context),
        "command.get_approximate_altitude_over_celestial_body" => {
            get_approximate_altitude_over_celestial_body(payload, context)
        }
        "command.get_real_altitude_over_celestial_body" => {
            get_real_altitude_over_celestial_body(payload, context)
        }
        "command.get_closest_celestial_body" => get_closest_celestial_body(payload, context),
        "command.get_gravity_flux" => get_gravity_flux(payload, context),
        "command.get_universe_clock" => get_universe_clock(payload, context),
        "command.set_universe_clock" => set_universe_clock(payload, context),
        "command.clear_universe_clock" => clear_universe_clock(payload, context),
        "command.get_first_person_camera_control" => {
            get_first_person_camera_control(payload, context)
        }
        "command.set_first_person_camera_control" => {
            set_first_person_camera_control(payload, context)
        }
        "command.clear_first_person_camera_control" => {
            clear_first_person_camera_control(payload, context)
        }
        "command.get_third_person_orbit_camera_control" => {
            get_third_person_orbit_camera_control(payload, context)
        }
        "command.set_third_person_orbit_camera_control" => {
            set_third_person_orbit_camera_control(payload, context)
        }
        "command.clear_third_person_orbit_camera_control" => {
            clear_third_person_orbit_camera_control(payload, context)
        }
        "command.get_third_person_static_camera_control" => {
            get_third_person_static_camera_control(payload, context)
        }
        "command.set_third_person_static_camera_control" => {
            set_third_person_static_camera_control(payload, context)
        }
        "command.clear_third_person_static_camera_control" => {
            clear_third_person_static_camera_control(payload, context)
        }
        "command.get_transform" => get_transform(payload, context),
        "command.set_transform" => set_transform(payload, context),
        "command.clear_transform" => clear_transform(payload, context),
        "command.get_real_physics" => get_real_physics(payload, context),
        "command.set_real_physics" => set_real_physics(payload, context),
        "command.clear_real_physics" => clear_real_physics(payload, context),
        "command.get_simple_physics" => get_simple_physics(payload, context),
        "command.set_simple_physics" => set_simple_physics(payload, context),
        "command.clear_simple_physics" => clear_simple_physics(payload, context),
        "command.get_glue_to_celestial_body" => get_glue_to_celestial_body(payload, context),
        "command.set_glue_to_celestial_body" => set_glue_to_celestial_body(payload, context),
        "command.clear_glue_to_celestial_body" => clear_glue_to_celestial_body(payload, context),
        "command.get_ship_control" => get_ship_control(payload, context),
        "command.set_ship_control" => set_ship_control(payload, context),
        "command.clear_ship_control" => clear_ship_control(payload, context),
        "command.get_ui_color" => get_ui_color(payload, context),
        "command.set_ui_color" => set_ui_color(payload, context),
        "command.clear_ui_color" => clear_ui_color(payload, context),
        "command.get_ui_hover_color" => get_ui_hover_color(payload, context),
        "command.set_ui_hover_color" => set_ui_hover_color(payload, context),
        "command.clear_ui_hover_color" => clear_ui_hover_color(payload, context),
        "command.get_ui_box" => get_ui_box(payload, context),
        "command.set_ui_box" => set_ui_box(payload, context),
        "command.clear_ui_box" => clear_ui_box(payload, context),
        "command.get_ui_hover_cursor" => get_ui_hover_cursor(payload, context),
        "command.set_ui_hover_cursor" => set_ui_hover_cursor(payload, context),
        "command.clear_ui_hover_cursor" => clear_ui_hover_cursor(payload, context),
        "command.get_ui_texture" => get_ui_texture(payload, context),
        "command.set_ui_texture" => set_ui_texture(payload, context),
        "command.clear_ui_texture" => clear_ui_texture(payload, context),
        "command.get_ui_text" => get_ui_text(payload, context),
        "command.set_ui_text" => set_ui_text(payload, context),
        "command.clear_ui_text" => clear_ui_text(payload, context),
        "command.get_set_physics_kinematics" => get_set_physics_kinematics(payload, context),
        "command.add_set_physics_kinematics" => add_set_physics_kinematics(payload, context),
        "command.remove_set_physics_kinematics" => remove_set_physics_kinematics(payload, context),
        "command.get_mesh" => get_mesh(payload, context),
        "command.add_mesh" => add_mesh(payload, context),
        "command.remove_mesh" => remove_mesh(payload, context),
        "command.get_camera_focus" => get_camera_focus(payload, context),
        "command.set_camera_focus" => set_camera_focus(payload, context),
        "command.get_is_ground_collider" => get_is_ground_collider(payload, context),
        "command.set_is_ground_collider" => set_is_ground_collider(payload, context),
        "command.get_is_celestial_body_surface" => get_is_celestial_body_surface(payload, context),
        "command.set_is_celestial_body_surface" => set_is_celestial_body_surface(payload, context),
        "command.get_is_player" => get_is_player(payload, context),
        "command.set_is_player" => set_is_player(payload, context),
        "command.get_control_focus" => get_control_focus(payload, context),
        "command.set_control_focus" => set_control_focus(payload, context),
        "command.get_ui_is_raycastable" => get_ui_is_raycastable(payload, context),
        "command.set_ui_is_raycastable" => set_ui_is_raycastable(payload, context),
        "command.get_ui_require_free_cursor" => get_ui_require_free_cursor(payload, context),
        "command.set_ui_require_free_cursor" => set_ui_require_free_cursor(payload, context),
        _ => Err(format!("Handler not found for message {}", name)),
    }
}
