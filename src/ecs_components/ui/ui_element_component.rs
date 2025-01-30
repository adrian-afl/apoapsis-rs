use crate::ecs::component_trait::component_type;
use crate::ecs::component_trait::ComponentTypes;
use crate::ecs::component_trait::{acquire_next_id, ComponentTrait};
use crate::impl_component;
use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UIFontSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UICursorType {
    Arrow,
    Grab,
    Pointer,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UIElementType {
    Rectangle {
        size: DVec2,
        color: DVec3,
    },
    Image {
        texture_path: String,
        size: DVec2,
    },
    Text {
        content: String,
        font_size: UIFontSize,
        color: DVec3,
    },
    Cursor {
        typ: UICursorType,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UIElementComponent {
    id: u64,
    typ: UIElementType,
    position: DVec2,
    z_index: i32,
    hover_cursor: UICursorType,
}

impl_component!(UIElementComponent, true);
