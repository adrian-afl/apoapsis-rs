use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::ComponentTrait;
use crate::ecs::component_trait::ComponentTypes;
use crate::ecs_components::ui::ui_cursor_component::UICursorType;
use crate::impl_component;
use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIHoverCursorComponent {
    id: u64,
    typ: UICursorType,
}

impl_component!(UIHoverCursorComponent, false);
