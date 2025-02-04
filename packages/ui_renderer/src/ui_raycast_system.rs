use ecs::component_trait::Components;
use ecs::ecs_world::ECSWorld;
use glam::DVec2;

#[derive(Debug)]
pub struct UIRaycastResultItem {
    pub entity_id: u64,
    pub z_index: i32,
}

pub struct UIRaycastSystem {}

impl UIRaycastSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(
        &mut self,
        raycast_result: &mut Vec<UIRaycastResultItem>,
        ecs: &mut ECSWorld,
        cursor_pos: DVec2,
    ) {
        println!("UIRaycastSystem / update");

        raycast_result.clear();

        ecs.process_all_by_components_mut(
            &[&Components::UIBox, &Components::UIIsRaycastable],
            |entity| {
                let uibox = entity.components.ui_box.as_ref().unwrap();
                if cursor_pos.x >= uibox.position.x
                    && cursor_pos.y >= uibox.position.y
                    && cursor_pos.x <= uibox.position.x + uibox.size.x
                    && cursor_pos.y <= uibox.position.y + uibox.size.y
                {
                    raycast_result.push(UIRaycastResultItem {
                        entity_id: entity.id,
                        z_index: uibox.z_index,
                    })
                }
            },
        );

        raycast_result.sort_by(|a, b| a.z_index.partial_cmp(&b.z_index).unwrap());

        println!("{:?}", *raycast_result);
    }
}
