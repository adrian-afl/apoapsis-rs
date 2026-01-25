use ecs::component_trait::Components;
use ecs::ecs_world::ECSWorld;
use glam::{DQuat, DVec2, DVec3, EulerRot};
use input::controls::Controls;
use math::decimal_vector_3d::DecimalVector3d;
use renderer_common::camera::Camera;
use std::f64::consts::PI;
/*
lets think about it
i think each camera system is coupled with a movement style
maybe it should then be called differently

lets imagine first from controlmethod
FirstPersonNearSurface
FirstPersonInSpace
ThirdPersonVehicleNearSurface
ThirdPersonVehicleInSpace

so this logic actually boils down to simpler:
if(vehicle){
    if (near space){
        ...
    }
} else if (on foot){
    if (near space){
        ...
    }
}

which means this needs:
FP normal camera rotating for Near Surface First Person
TP orbit camera rotating for Near Surface Vehicle
FP special camera that allows 3rd axis to rotate for In Space First Person
Chase rotating TP camera for In Space Vehicle

Before that surface collision must be implemented first
 */

pub struct CameraSystem {
    last_mouse: DVec2,
    deflection: DVec2,
}

impl CameraSystem {
    pub fn new() -> Self {
        Self {
            last_mouse: DVec2::ZERO,
            deflection: DVec2::ZERO,
        }
    }

    pub fn update(&mut self, camera: &mut Camera, controls: &Controls, ecs: &mut ECSWorld) {
        // println!("CameraSystem / update");

        let entity =
            ecs.find_first_by_components(&[&Components::CameraFocus, &Components::Transform]);
        match entity {
            Some(entity) => {
                if entity.components.has(&Components::FirstPersonCameraControl) {
                    let transform = entity.components.transform.as_ref().unwrap();
                    let mouse = controls.mouse.get_cursor_integrated();
                    let mouse_diff = mouse - self.last_mouse;
                    self.last_mouse = mouse;

                    // transform.orientation *= DQuat::from_axis_angle(DVec3::X, mouse_diff.y)
                    //     * DQuat::from_axis_angle(DVec3::Y, mouse_diff.x);

                    let first_person_component = entity
                        .components
                        .first_person_camera_control
                        .as_ref()
                        .unwrap();
                    camera.position.assign(&transform.position);
                    self.deflection += -mouse_diff * 0.001;
                    self.deflection.y = self.deflection.y.clamp(-PI / 2.0, PI / 2.0);
                    self.deflection.x = self.deflection.x % (PI * 2.0);
                    // let mouse_rot = DQuat::from_axis_angle(DVec3::Y, self.deflection.x)
                    //     * DQuat::from_axis_angle(DVec3::X, (self.deflection.y).clamp(-PI, PI));
                    let mouse_rot =
                        DQuat::from_euler(EulerRot::ZYX, self.deflection.y, self.deflection.x, 0.0);
                    // CHASE
                    // camera.orientation = transform.orientation.inverse() * mouse_rot;
                    // ABSOLTUE
                    camera.orientation = mouse_rot;
                    camera.set_perspective(
                        first_person_component.fov * (PI / 180.0),
                        4.0 / 3.0,
                        0.1,
                        9999999999.0,
                    );

                    camera.update();

                    // println!("Cam pos is now {}", camera.position);
                }
                if entity
                    .components
                    .has(&Components::ThirdPersonOrbitCameraControl)
                {
                    let transform = entity.components.transform.as_ref().unwrap();
                    let mouse = controls.mouse.get_cursor_integrated();
                    let mouse_diff = mouse - self.last_mouse;
                    self.last_mouse = mouse;

                    // transform.orientation *= DQuat::from_axis_angle(DVec3::X, mouse_diff.y)
                    //     * DQuat::from_axis_angle(DVec3::Y, mouse_diff.x);

                    let camcontrol = entity
                        .components
                        .third_person_orbit_camera_control
                        .as_ref()
                        .unwrap();
                    // CHASE

                    self.deflection += -mouse_diff * 0.001;
                    self.deflection.y = self.deflection.y.clamp(-PI / 2.0, PI / 2.0);
                    self.deflection.x = self.deflection.x % (PI * 2.0);
                    // let mouse_rot = DQuat::from_axis_angle(DVec3::Y, self.deflection.x)
                    //     * DQuat::from_axis_angle(DVec3::X, (self.deflection.y).clamp(-PI, PI));
                    let mouse_rot =
                        DQuat::from_euler(EulerRot::ZYX, self.deflection.y, self.deflection.x, 0.0);

                    camera.position.assign(
                        &(&transform.position
                            + &DecimalVector3d::from_dvec3(mouse_rot * -camcontrol.initial_offset)),
                    );
                    camera.orientation = mouse_rot;
                    camera.set_perspective(
                        camcontrol.fov * (PI / 180.0),
                        4.0 / 3.0,
                        0.1,
                        9999999999.0,
                    );

                    camera.update();

                    // println!("Cam pos is now {}", camera.position);
                }
            }
            None => {
                // println!(
                //     "Cannot update camera as there is no entity with CameraFocus and Transform"
                // );
            }
        }
    }
}
