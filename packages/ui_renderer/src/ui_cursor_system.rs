use crate::ui_raycast_system::UIRaycastResultItem;
use ecs::component_trait::Components;
use ecs::components::ui::cursor_type::UICursorType;
use ecs::ecs_world::ECSWorld;

pub struct UICursorSystemResult {
    pub cursor_locked: bool,
    pub cursor_type: UICursorType,
}

pub struct UICursorSystem {}

impl UICursorSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(
        &mut self,
        ecs: &mut ECSWorld,
        raycast_result: &Vec<UIRaycastResultItem>,
    ) -> UICursorSystemResult {
        // println!("UICursorSystem / update");

        let free_cursor_entity = ecs.find_first_by_components(&[&Components::UIRequireFreeCursor]);
        let cursor_locked = free_cursor_entity.is_none();

        let mut cursor_type = UICursorType::Arrow;

        // from bottom to top only components with cursor, it will be fine
        for item in raycast_result {
            let entity = ecs.find_by_id(item.entity_id).unwrap();
            if let Some(cursor) = &entity.components.ui_hover_cursor {
                cursor_type = cursor.typ.clone();
            }
        }

        UICursorSystemResult {
            cursor_locked,
            cursor_type,
        }
    }
}
