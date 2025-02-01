use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::ComponentTrait;
use crate::ecs::component_trait::ComponentTypes;
use crate::impl_component;
use glam::DVec4;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIColorComponent {
    id: u64,
    color: DVec4,
}

impl_component!(UIColorComponent, false);
