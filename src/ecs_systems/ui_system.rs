use crate::celestial_rendering::errors::RenderingError;
use crate::config::Config;
use crate::core::game_state::GameState;
use crate::ecs::component_trait::Components;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_components::ui::cursor_type::UICursorType;
use crate::ui_rendering::ui_drawer::UIDrawer;
use crate::ui_rendering::ui_rendered_item::UIRenderedItem;
use glam::DVec2;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use thiserror::Error;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::VEImageUsage;

pub struct UIRenderer {
    config: Config,

    toolkit: Arc<VEToolkit>,

    pub ui_drawer: Arc<Mutex<UIDrawer>>,
    currently_rendered_items: RwLock<HashMap<u64, UIRenderedItem>>,
}

#[derive(Error, Debug)]
pub enum UIRendererError {
    #[error("rendering error")]
    RenderingError(#[from] RenderingError),
}

impl UIRenderer {
    pub fn new(toolkit: Arc<VEToolkit>, config: &Config) -> Self {
        Self {
            config: config.clone(),
            toolkit: toolkit.clone(),
            ui_drawer: Arc::new(Mutex::from(
                UIDrawer::new(&config, &toolkit).expect("Failed to create UIDrawer"),
            )),
            currently_rendered_items: RwLock::new(HashMap::new()),
        }
    }
}

impl SystemTrait for UIRenderer {
    fn update(&mut self, game_state: Arc<Mutex<GameState>>, ecs: Arc<Mutex<ECSWorld>>) {
        println!("UIRenderer / update");

        let cursor_pos = DVec2::new(0.0, 0.0); // TODO
        let mut cursor_type = Mutex::from(UICursorType::Arrow);

        let ecs = ecs.lock().unwrap();
        let detected_entity_ids = Mutex::new(vec![]);
        ecs.parallel_process_all_by_components(&[&Components::UIBox], |entity| {
            detected_entity_ids.lock().unwrap().push(entity.id);
            //
            let exists = self
                .currently_rendered_items
                .try_read()
                .unwrap()
                .contains_key(&entity.id);

            if !exists {
                self.currently_rendered_items.try_write().unwrap().insert(
                    entity.id,
                    UIRenderedItem::empty(
                        &self.toolkit,
                        &mut self.ui_drawer.lock().unwrap().item_set_layout,
                    )
                    .unwrap(),
                );
            }

            let mut map_locked = self.currently_rendered_items.try_write().unwrap();

            let item = map_locked.get_mut(&entity.id).unwrap();

            if let Some(texture) = &entity.components.ui_texture {
                if item.texture_component_id != Some(texture.id) {
                    item.texture = Some(
                        self.toolkit
                            .create_image_from_file(&texture.texture_path, &[VEImageUsage::Sampled])
                            .unwrap(),
                    );
                }
            } else {
                item.texture_component_id = None;
                item.texture = None;
            }

            let uibox = &entity.components.ui_box.as_ref().unwrap();

            if let Some(color) = &entity.components.ui_color {
                item.color = color.color;
            }

            if let Some(color) = &entity.components.ui_hover_color {
                if cursor_pos.x >= uibox.position.x
                    && cursor_pos.y >= uibox.position.y
                    && cursor_pos.x <= uibox.position.x + uibox.size.x
                    && cursor_pos.y <= uibox.position.y + uibox.size.y
                {
                    item.color = color.color;
                }
            }

            if let Some(cursor) = &entity.components.ui_hover_cursor {
                if cursor_pos.x >= uibox.position.x
                    && cursor_pos.y >= uibox.position.y
                    && cursor_pos.x <= uibox.position.x + uibox.size.x
                    && cursor_pos.y <= uibox.position.y + uibox.size.y
                {
                    *cursor_type.lock().unwrap() = cursor.typ.clone();
                }
            }
        });

        let locked_map = self.currently_rendered_items.try_read().unwrap();
        let detected_entity_ids = detected_entity_ids.lock().unwrap();
        let keys: Vec<&u64> = locked_map.keys().collect();

        keys.par_iter().for_each(|key| {
            if !detected_entity_ids.contains(key) {
                let mut locked_map = self.currently_rendered_items.try_write().unwrap();
                locked_map.remove(key);
            }
        });

        let drawer = self.ui_drawer.lock().unwrap();
        drawer
            .record(
                &self
                    .currently_rendered_items
                    .try_read()
                    .unwrap()
                    .values()
                    .collect::<Vec<_>>(),
            )
            .unwrap();
    }
}
