use crate::errors::PhysicsError;
use glam::{DQuat, DVec3};
use rapier3d_f64::na::{Quaternion, SMatrix};
use rapier3d_f64::prelude::*;

use rapier3d_f64::pipeline::DebugRenderPipeline;
use rapier3d_f64::pipeline::DebugRenderStyle;
use serde::{Deserialize, Serialize};
use tcpapi::send_event;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct OnPhysicsCollisionEventData {
    pub entity_a: u64,
    pub entity_b: u64,
    pub total_impulse_magnitude: f64,
}

// @api_event on_physics_collision_event(OnPhysicsCollisionEventData)

pub struct MyEventHandler {}

impl EventHandler for MyEventHandler {
    fn handle_collision_event(
        &self,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        event: CollisionEvent,
        contact_pair: Option<&ContactPair>,
    ) {
        match contact_pair {
            None => {}
            Some(contact_pair) => {
                let c1 = colliders.get(contact_pair.collider1).unwrap();
                let c2 = colliders.get(contact_pair.collider2).unwrap();
                let b1 = bodies.get(c1.parent().unwrap()).unwrap();
                let b2 = bodies.get(c2.parent().unwrap()).unwrap();
                let entity_a = b1.user_data as u64;
                let entity_b = b2.user_data as u64;

                let total_impulse_magnitude = contact_pair.total_impulse_magnitude();

                send_event!(
                    "on_physics_collision_event",
                    OnPhysicsCollisionEventData {
                        entity_a,
                        entity_b,
                        total_impulse_magnitude
                    }
                );
            }
        }

        // match contact_pair {
        //     None => {
        //         println!("handle_collision_event {:?}", event);
        //     }
        //     Some(contact_pair) => {
        //         println!(
        //             "handle_collision_event {:?}, {:?}, {:?}, {:?}",
        //             event, contact_pair.collider1, contact_pair.collider2, contact_pair.manifolds
        //         );
        //     }
        // }
    }

    fn handle_contact_force_event(
        &self,
        dt: f64,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        contact_pair: &ContactPair,
        total_force_magnitude: f64,
    ) {
        // println!(
        //     "handle_contact_force_event  {:?}, {:?}, {:?}, {:?}, {:?}",
        //     dt,
        //     contact_pair.collider1,
        //     contact_pair.collider2,
        //     contact_pair.manifolds,
        //     total_force_magnitude
        // );
    }
}

pub struct RealPhysicsSystem {
    gravity: Vector,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    debug_render_pipeline: DebugRenderPipeline,
    my_event_handler: MyEventHandler,
}

pub struct RealPhysicsBodyKinematics {
    pub position: DVec3,
    pub orientation: DQuat,
    pub linear_velocity: DVec3,
    pub angular_velocity: DVec3,
}

pub struct SetRealPhysicsBodyKinematics {
    pub position: Option<DVec3>,
    pub orientation: Option<DQuat>,
    pub linear_velocity: Option<DVec3>,
    pub angular_velocity: Option<DVec3>,
    pub wake_up: bool,
}

pub struct RealPhysicsColliderKinematics {
    pub position: DVec3,
    pub orientation: DQuat,
}

pub struct SetRealPhysicsColliderKinematics {
    pub position: Option<DVec3>,
    pub orientation: Option<DQuat>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct DebugCollector {
    lines: Vec<[f64; 3]>,
    colors: Vec<[f32; 4]>,
}

impl DebugRenderBackend for DebugCollector {
    fn filter_object(&self, _object: DebugRenderObject) -> bool {
        true
    }

    fn draw_line(&mut self, object: DebugRenderObject, a: Vector, b: Vector, color: [f32; 4]) {
        self.lines.push([a.x, a.y, a.z]);
        self.lines.push([b.x, b.y, b.z]);
        self.colors.push(color);
    }

    fn draw_polyline(
        &mut self,
        object: DebugRenderObject,
        vertices: &[Vector],
        indices: &[[u32; 2]],
        transform: &Pose,
        scale: Vector,
        color: [f32; 4],
    ) {
        for index in indices {
            let mut a = (transform * vertices[index[0] as usize]);
            a.x *= scale.x;
            a.y *= scale.y;
            a.z *= scale.z;

            let mut b = (transform * vertices[index[1] as usize]);
            b.x *= scale.x;
            b.y *= scale.y;
            b.z *= scale.z;

            self.lines.push([a.x, a.y, a.z]);
            self.lines.push([b.x, b.y, b.z]);
            self.colors.push(color);
        }
    }

    fn draw_line_strip(
        &mut self,
        object: DebugRenderObject,
        vertices: &[Vector],
        transform: &Pose,
        scale: Vector,
        color: [f32; 4],
        closed: bool,
    ) {
        let len = vertices.len();
        for i in 0..len - 1 {
            let mut v = transform * vertices[i];
            v.x *= scale.x;
            v.y *= scale.y;
            v.z *= scale.z;
            self.lines.push([v.x, v.y, v.z]);

            let mut v = transform * vertices[i + 1];
            v.x *= scale.x;
            v.y *= scale.y;
            v.z *= scale.z;
            self.lines.push([v.x, v.y, v.z]);
            self.colors.push(color);
        }
    }
}

impl RealPhysicsSystem {
    pub fn new() -> RealPhysicsSystem {
        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();

        // /* Create the ground. */
        // let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
        // collider_set.insert(collider);
        //
        // /* Create the bounding ball. */
        // let rigid_body = RigidBodyBuilder::dynamic()
        //     .translation(vector![0.0, 10.0, 0.0])
        //     .build();
        // let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        // let ball_body_handle = rigid_body_set.insert(rigid_body);
        // collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        let gravity = Vector::new(0.0, 0.0, 0.0); // REMEMBER ABOUT THIS
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let my_event_handler = MyEventHandler {};

        RealPhysicsSystem {
            rigid_body_set,
            collider_set,
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            debug_render_pipeline: DebugRenderPipeline::default(),
            my_event_handler,
        }
    }

    pub fn debug_get_world(&mut self) -> DebugCollector {
        let mut collector = DebugCollector {
            lines: vec![],
            colors: vec![],
        };

        self.debug_render_pipeline.render(
            &mut collector,
            &self.rigid_body_set,
            &self.collider_set,
            &self.impulse_joint_set,
            &self.multibody_joint_set,
            &self.narrow_phase,
        );

        collector
    }

    pub fn step(&mut self, delta: f64) {
        // dbg!(delta);
        self.integration_parameters.dt = delta;
        self.integration_parameters.max_ccd_substeps = 1;
        self.physics_pipeline.step(
            self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            // TODO events, especially collision!
            &(),                    //&self.physics_hooks,
            &self.my_event_handler, //&self.event_handler,
        );
    }

    // pub fn add_collider(&mut self, collider: Collider) -> ColliderHandle {
    //     self.collider_set.insert(collider)
    // }

    pub fn add_body_with_collider(
        &mut self,
        body: RigidBody,
        collider: Collider,
    ) -> (RigidBodyHandle, ColliderHandle) {
        let body_handle = self.rigid_body_set.insert(body);
        let collider_handle =
            self.collider_set
                .insert_with_parent(collider, body_handle, &mut self.rigid_body_set);
        (body_handle, collider_handle)
    }

    // pub fn remove_collider(&mut self, collider_handle: ColliderHandle) {
    //     // TODO maybe wakeup should be controllable, but why? it doesn't make sense to wake a body without collider
    //     self.collider_set.remove(
    //         collider_handle,
    //         &mut self.island_manager,
    //         &mut self.rigid_body_set,
    //         false,
    //     );
    // }

    pub fn remove_body(&mut self, body_handle: RigidBodyHandle) {
        self.rigid_body_set.remove(
            body_handle,
            &mut self.island_manager,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true,
        );
    }

    pub fn get_body_kinematics(
        &self,
        body_handle: RigidBodyHandle,
    ) -> Result<RealPhysicsBodyKinematics, PhysicsError> {
        match self.rigid_body_set.get(body_handle) {
            None => Err(PhysicsError::RigidBodyNotFound),
            Some(body) => {
                // let translation = body.translation();
                let position = body.position();
                // let orientation = body.rotation();
                let linear_velocity = body.linvel();
                let angular_velocity = body.angvel();
                Ok(RealPhysicsBodyKinematics {
                    position: DVec3::new(
                        position.translation.x,
                        position.translation.y,
                        position.translation.z,
                    ),
                    orientation: DQuat::from_xyzw(
                        position.rotation.x,
                        position.rotation.y,
                        position.rotation.z,
                        position.rotation.w,
                    ),
                    linear_velocity: DVec3::new(
                        linear_velocity.x,
                        linear_velocity.y,
                        linear_velocity.z,
                    ),
                    angular_velocity: DVec3::new(
                        angular_velocity.x,
                        angular_velocity.y,
                        angular_velocity.z,
                    ),
                })
            }
        }
    }

    pub fn set_body_kinematics(
        &mut self,
        body_handle: RigidBodyHandle,
        data: SetRealPhysicsBodyKinematics,
    ) -> Result<(), PhysicsError> {
        match self.rigid_body_set.get_mut(body_handle) {
            None => Err(PhysicsError::RigidBodyNotFound),
            Some(body) => {
                if let Some(position) = data.position {
                    body.set_translation(Vector::new(position.x, position.y, position.z), false);
                }
                if let Some(orientation) = data.orientation {
                    body.set_rotation(
                        Quaternion::new(orientation.w, orientation.x, orientation.y, orientation.z)
                            .into(),
                        false,
                    );
                }
                if let Some(linear_velocity) = data.linear_velocity {
                    body.set_linvel(
                        Vector::new(linear_velocity.x, linear_velocity.y, linear_velocity.z),
                        false,
                    );
                }
                if let Some(angular_velocity) = data.angular_velocity {
                    body.set_angvel(
                        Vector::new(angular_velocity.x, angular_velocity.y, angular_velocity.z),
                        false,
                    );
                }
                if data.wake_up {
                    body.wake_up(true) // strong??
                }
                Ok(())
            }
        }
    }

    pub fn get_collider_kinematics(
        &self,
        collider_handle: ColliderHandle,
    ) -> Result<RealPhysicsColliderKinematics, PhysicsError> {
        match self.collider_set.get(collider_handle) {
            None => Err(PhysicsError::ColliderNotFound),
            Some(collider) => {
                let translation = collider.translation();
                let orientation = collider.rotation();
                Ok(RealPhysicsColliderKinematics {
                    position: DVec3::new(translation.x, translation.y, translation.z),
                    orientation: DQuat::from_xyzw(
                        orientation.x,
                        orientation.y,
                        orientation.z,
                        orientation.w,
                    ),
                })
            }
        }
    }

    pub fn set_collider_kinematics(
        &mut self,
        collider_handle: ColliderHandle,
        data: SetRealPhysicsBodyKinematics,
    ) -> Result<(), PhysicsError> {
        match self.collider_set.get_mut(collider_handle) {
            None => Err(PhysicsError::ColliderNotFound),
            Some(collider) => {
                if let Some(position) = data.position {
                    collider.set_translation(Vector::new(position.x, position.y, position.z));
                }
                if let Some(orientation) = data.orientation {
                    collider.set_rotation(
                        Quaternion::new(orientation.w, orientation.x, orientation.y, orientation.z)
                            .into(),
                    );
                };
                Ok(())
            }
        }
    }

    pub fn apply_impulse(
        &mut self,
        body_handle: RigidBodyHandle,
        impulse: DVec3,
        wake_up: bool,
    ) -> Result<(), PhysicsError> {
        match self.rigid_body_set.get_mut(body_handle) {
            None => Err(PhysicsError::RigidBodyNotFound),
            Some(body) => {
                body.apply_impulse(Vector::new(impulse.x, impulse.y, impulse.z), wake_up);
                Ok(())
            }
        }
    }

    pub fn apply_force(
        &mut self,
        body_handle: RigidBodyHandle,
        force: DVec3,
        wake_up: bool,
    ) -> Result<(), PhysicsError> {
        match self.rigid_body_set.get_mut(body_handle) {
            None => Err(PhysicsError::RigidBodyNotFound),
            Some(body) => {
                body.reset_forces(wake_up);
                body.add_force(Vector::new(force.x, force.y, force.z), wake_up);
                Ok(())
            }
        }
    }

    pub fn set_global_gravity(&mut self, gravity: DVec3) -> () {
        self.gravity = Vector::new(gravity.x, gravity.y, gravity.z);
    }

    pub fn raycast(&self, camera_relative_origin: DVec3, direction: DVec3) -> Option<f64> {
        let query_pipeline = self.broad_phase.as_query_pipeline(
            self.narrow_phase.query_dispatcher(),
            &self.rigid_body_set,
            &self.collider_set,
            QueryFilter::default(),
        );
        let result = query_pipeline.cast_ray(
            &Ray::new(
                Vector::new(
                    camera_relative_origin.x,
                    camera_relative_origin.y,
                    camera_relative_origin.z,
                ),
                Vector::new(direction.x, direction.y, direction.z),
            ),
            f64::MAX,
            false,
        );

        result.map(|x| x.1)
    }
}
