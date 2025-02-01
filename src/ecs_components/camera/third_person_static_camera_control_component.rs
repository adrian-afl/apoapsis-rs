use crate::ecs::component_trait::ComponentTypes;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThirdPersonStaticCameraControlComponent {
    pub id: u64,
    pub offset: DVec3,
    pub orientation: DQuat,
}

impl_component!(ThirdPersonStaticCameraControlComponent);

impl ThirdPersonStaticCameraControlComponent {
    pub fn new(offset: DVec3, orientation: DQuat) -> Self {
        Self {
            id: acquire_next_id(),
            offset,
            orientation,
        }
    }
}
