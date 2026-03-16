use crate::errors::PhysicsError;
use glam::{DQuat, DVec3};
use katana_physics::integrator::katana_integrate::katana_integrate;
use katana_physics::katana_raycast::katana_raycast;
use katana_physics::katana_rigid_body::KatanaRigidBody;
use katana_physics::katana_world::{KatanaWorld, KatanaWorldBodies};
use katana_physics::plugins::collision_solver_plugin::katana_collision_solver_plugin::KatanaCollisionSolverPlugin;
use katana_physics::plugins::katana_plugin::{BoxedKatanaPlugin, KatanaPlugin};
use serde::{Deserialize, Serialize};
use tcpapi::send_event;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct OnPhysicsCollisionEventData {
    pub entity_a: u64,
    pub entity_b: u64,
    #[ts(type = "[number, number, number]")]
    pub center_ws: DVec3,
    #[ts(type = "[number, number, number]")]
    pub normal_ws: DVec3,
    pub overlap: f64,
}

// @api_event on_physics_collision_event(OnPhysicsCollisionEventData)

fn handle_collision_events(
    bodies: &KatanaWorldBodies,
    collision_plugin: &KatanaCollisionSolverPlugin,
) {
    for collision in collision_plugin.get_last_collisions() {
        let body_a = bodies.get(collision.a_id).unwrap();
        let body_b = bodies.get(collision.b_id).unwrap();
        let entity_a = body_a.user_data as u64;
        let entity_b = body_b.user_data as u64;

        send_event!(
            "on_physics_collision_event",
            OnPhysicsCollisionEventData {
                entity_a,
                entity_b,
                center_ws: collision.manifold.center_ws,
                normal_ws: collision.manifold.normal_ws,
                overlap: collision.manifold.overlap,
            }
        );
    }
}

pub struct RealPhysicsSystem {
    gravity: DVec3,
    katana_world: KatanaWorld,
    plugins: Vec<BoxedKatanaPlugin>,
    // collision_plugin: &'a KatanaCollisionSolverPlugin,
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

impl RealPhysicsSystem {
    pub fn new() -> Self {
        let collision_plugin = KatanaCollisionSolverPlugin::new();
        let plugins: Vec<BoxedKatanaPlugin> = vec![Box::new(collision_plugin)];
        Self {
            gravity: DVec3::ZERO,
            katana_world: KatanaWorld::new(),
            // collision_plugin: &plugins[0],
            plugins,
        }
    }

    pub fn step(&mut self, delta: f64) {
        katana_integrate(delta, &mut self.katana_world, &mut self.plugins)
    }

    pub fn add_body(&mut self, body: KatanaRigidBody) -> u64 {
        self.katana_world.bodies.add(body)
    }

    pub fn remove_body(&mut self, body_id: u64) -> Option<KatanaRigidBody> {
        self.katana_world.bodies.remove(body_id)
    }

    pub fn get_body_kinematics(
        &self,
        body_id: u64,
    ) -> Result<RealPhysicsBodyKinematics, PhysicsError> {
        match self.katana_world.bodies.get(body_id) {
            None => Err(PhysicsError::RigidBodyNotFound),
            Some(body) => Ok(RealPhysicsBodyKinematics {
                position: body.position,
                orientation: body.orientation,
                linear_velocity: body.linear_velocity,
                angular_velocity: body.angular_velocity,
            }),
        }
    }

    pub fn set_body_kinematics(
        &mut self,
        body_id: u64,
        data: SetRealPhysicsBodyKinematics,
    ) -> Result<(), PhysicsError> {
        match self.katana_world.bodies.get_mut(body_id) {
            None => Err(PhysicsError::RigidBodyNotFound),
            Some(body) => {
                if let Some(position) = data.position {
                    body.position = position;
                }
                if let Some(orientation) = data.orientation {
                    body.orientation = orientation;
                }
                if let Some(linear_velocity) = data.linear_velocity {
                    body.linear_velocity = linear_velocity;
                }
                if let Some(angular_velocity) = data.angular_velocity {
                    body.angular_velocity = angular_velocity;
                }
                if data.wake_up {
                    body.wake_up();
                }
                Ok(())
            }
        }
    }

    pub fn apply_impulse(
        &mut self,
        body_id: u64,
        position: DVec3,
        impulse: DVec3,
    ) -> Result<(), PhysicsError> {
        match self.katana_world.bodies.get_mut(body_id) {
            None => Err(PhysicsError::RigidBodyNotFound),
            Some(body) => {
                body.apply_impulse(position, impulse);
                Ok(())
            }
        }
    }

    pub fn apply_force(&mut self, body_id: u64, force: DVec3) -> Result<(), PhysicsError> {
        match self.katana_world.bodies.get_mut(body_id) {
            None => Err(PhysicsError::RigidBodyNotFound),
            Some(body) => {
                body.linear_velocity += force;
                Ok(())
            }
        }
    }

    pub fn set_global_gravity(&mut self, gravity: DVec3) -> () {
        self.gravity = gravity;
    }

    pub fn raycast(&self, camera_relative_origin: DVec3, direction: DVec3) -> Option<f64> {
        Some(
            katana_raycast(
                &self.katana_world.bodies,
                camera_relative_origin,
                direction,
                &[],
                999999999.0,
            )
            .closest?
            .distance,
        )
    }
}
