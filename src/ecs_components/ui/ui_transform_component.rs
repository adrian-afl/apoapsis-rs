use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::ComponentTrait;
use crate::ecs::component_trait::ComponentTypes;
use crate::impl_component;
use glam::DVec2;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UITransformComponent {
    id: u64,
    position: DVec2,
    orientation: f64, // radians
    z_index: i32,
}

impl_component!(UITransformComponent, false);
