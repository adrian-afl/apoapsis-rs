use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::ComponentTrait;
use crate::ecs::component_trait::ComponentTypes;
use crate::impl_component;
use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UICursorType {
    Arrow,
    Grab,
    Pointer,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UICursorComponent {
    id: u64,
    typ: UICursorType,
}

impl_component!(UICursorComponent, false);
