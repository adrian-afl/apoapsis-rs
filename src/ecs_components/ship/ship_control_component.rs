use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::ComponentsTypes;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShipControlComponent {
    pub id: u64,
    pub linear_impulse_strength: f64,
    pub angular_impulse_strength: f64,
}

impl_component!(ShipControlComponent, false);

impl ShipControlComponent {
    pub fn new(linear_impulse_strength: f64, angular_impulse_strength: f64) -> Self {
        Self {
            id: acquire_next_id(),
            linear_impulse_strength,
            angular_impulse_strength,
        }
    }
}
