use crate::errors::PhysicsError;
use glam::{DQuat, DVec3};
use rapier3d_f64::na::{Quaternion, SMatrix};
use rapier3d_f64::prelude::*;

pub struct RealPhysicsSystem {
    gravity: SMatrix<f64, 3, 1>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
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

        let gravity = vector![0.0, 0.0, 0.0]; // REMEMBER ABOUT THIS
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let _physics_hooks = ();
        let _event_handler = ();

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
            query_pipeline,
        }
    }

    pub fn step(&mut self, delta: f64) {
        self.integration_parameters.dt = delta;
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            // TODO events, especially collision!
            &(), //&self.physics_hooks,
            &(), //&self.event_handler,
        );
    }

    pub fn add_collider(&mut self, collider: Collider) -> ColliderHandle {
        self.collider_set.insert(collider)
    }

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

    pub fn remove_collider(&mut self, collider_handle: ColliderHandle) {
        // TODO maybe wakeup should be controllable, but why? it doesn't make sense to wake a body without collider
        self.collider_set.remove(
            collider_handle,
            &mut self.island_manager,
            &mut self.rigid_body_set,
            false,
        );
    }

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
                let translation = body.translation();
                let orientation = body.rotation();
                let linear_velocity = body.linvel();
                let angular_velocity = body.angvel();
                Ok(RealPhysicsBodyKinematics {
                    position: DVec3::new(translation.x, translation.y, translation.z),
                    orientation: DQuat::from_xyzw(
                        orientation.i,
                        orientation.j,
                        orientation.k,
                        orientation.w,
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
                    body.set_translation(vector![position.x, position.y, position.z], false);
                }
                if let Some(orientation) = data.orientation {
                    body.set_rotation(
                        Rotation::from_quaternion(Quaternion::new(
                            orientation.w,
                            orientation.x,
                            orientation.y,
                            orientation.z,
                        )),
                        false,
                    );
                }
                if let Some(linear_velocity) = data.linear_velocity {
                    body.set_linvel(
                        vector![linear_velocity.x, linear_velocity.y, linear_velocity.z],
                        false,
                    );
                }
                if let Some(angular_velocity) = data.angular_velocity {
                    body.set_angvel(
                        vector![angular_velocity.x, angular_velocity.y, angular_velocity.z],
                        false,
                    );
                }
                if data.wake_up {
                    body.wake_up(false) // strong??
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
                        orientation.i,
                        orientation.j,
                        orientation.k,
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
                    collider.set_translation(vector![position.x, position.y, position.z]);
                }
                if let Some(orientation) = data.orientation {
                    collider.set_rotation(Rotation::from_quaternion(Quaternion::new(
                        orientation.w,
                        orientation.x,
                        orientation.y,
                        orientation.z,
                    )));
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
                body.apply_impulse(vector![impulse.x, impulse.y, impulse.z], wake_up);
                Ok(())
            }
        }
    }
}
