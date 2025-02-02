use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhysicsError {
    #[error("rigid body not found")]
    RigidBodyNotFound,

    #[error("collider not found")]
    ColliderNotFound,
}
