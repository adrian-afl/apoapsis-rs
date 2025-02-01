use crate::ecs::component_trait::ComponentTrait;
use crate::ecs::component_trait::ComponentTypes;
use crate::impl_component;
use glam::DVec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIRectangleComponent {
    id: u64,
    size: DVec2,
}

impl_component!(UIRectangleComponent);
