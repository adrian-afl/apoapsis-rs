use crate::ui_drawer::UIDrawer;
use crate::ui_rendered_item::UIRenderedItem;
use ecs::component_trait::Components;
use ecs::ecs_world::ECSWorld;
use glam::{DVec2, DVec4};
use rayon::iter::ParallelIterator;
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use thiserror::Error;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::VEImageUsage;

pub struct UISystem {
    config: ResolutionConfig,

    toolkit: Arc<VEToolkit>,

    pub ui_drawer: Arc<Mutex<UIDrawer>>,
    currently_rendered_items: RwLock<HashMap<u64, UIRenderedItem>>,
}

#[derive(Error, Debug)]
pub enum UIRendererError {
    #[error("rendering error")]
    RenderingError(#[from] RenderingError),
}

impl UISystem {
    pub fn new(toolkit: Arc<VEToolkit>, config: &ResolutionConfig) -> Self {
        let mut ui_drawer = UIDrawer::new(config, &toolkit).expect("Failed to create UIDrawer");
        ui_drawer.update_buffer().unwrap();
        Self {
            config: config.clone(),
            toolkit: toolkit.clone(),
            ui_drawer: Arc::new(Mutex::from(ui_drawer)),
            currently_rendered_items: RwLock::new(HashMap::new()),
        }
    }

    pub fn update(&mut self, ecs: &mut ECSWorld, cursor_pos: DVec2) {
        // println!("UIRenderer / update");

        let detected_entity_ids = Mutex::new(vec![]);
        // println!("UIRenderer / update A");
        ecs.parallel_process_all_by_components(&[&Components::UIBox], |entity| {
            detected_entity_ids.lock().unwrap().push(entity.id);
            //
            let exists = self
                .currently_rendered_items
                .read()
                .unwrap()
                .contains_key(&entity.id);

            if !exists {
                self.currently_rendered_items.write().unwrap().insert(
                    entity.id,
                    UIRenderedItem::empty(
                        &self.toolkit,
                        &mut self.ui_drawer.lock().unwrap().item_set_layout,
                    )
                    .unwrap(),
                );
            }

            let mut map_locked = self.currently_rendered_items.write().unwrap();

            let item = map_locked.get_mut(&entity.id).unwrap();

            if let Some(texture) = &entity.components.ui_texture {
                if item.texture_path_loaded.is_none()
                    || *item.texture_path_loaded.as_ref().unwrap() != texture.texture_path
                {
                    item.texture = Some(
                        self.toolkit
                            .create_image_from_file(&texture.texture_path, &[VEImageUsage::Sampled])
                            .unwrap(),
                    );
                    item.bind_texture();
                }
            } else {
                item.texture_path_loaded = None;
                item.texture = None;
            }

            let uibox = &entity.components.ui_box.as_ref().unwrap();

            item.position = uibox.position;
            item.size = uibox.size;
            item.orientation = uibox.orientation;
            item.z_index = uibox.z_index;

            if let Some(color) = &entity.components.ui_color {
                item.color = color.color;
            } else {
                item.color = DVec4::new(1.0, 1.0, 1.0, 1.0);
            }

            if let Some(color) = &entity.components.ui_hover_color
                && cursor_pos.x >= uibox.position.x
                    && cursor_pos.y >= uibox.position.y
                    && cursor_pos.x <= uibox.position.x + uibox.size.x
                    && cursor_pos.y <= uibox.position.y + uibox.size.y
                {
                    item.color = color.color;
                }

            if let Some(text) = &entity.components.ui_text {
                item.text = text.content.clone();
                item.text_color = text.color;
                item.font_size = text.font_size.clone();
            }

            let ui_drawer = self.ui_drawer.lock().unwrap();
            item.update_buffer(
                &ui_drawer.font_atlas_small,
                &ui_drawer.font_atlas_medium,
                &ui_drawer.font_atlas_large,
            )
            .unwrap();
        });
        // println!("UIRenderer / update B");

        let mut locked_map = self.currently_rendered_items.write().unwrap();

        let detected_entity_ids = detected_entity_ids.lock().unwrap();
        locked_map.retain(|k, _| detected_entity_ids.contains(k));
    }

    pub fn with_items<F>(&self, mut run: F)
    where
        F: FnMut(&[&UIRenderedItem]),
    {
        let locked_map = self.currently_rendered_items.read().unwrap();

        // println!("UIRenderer / update C");

        let items = locked_map.values().collect::<Vec<_>>();

        run(&items);
    }
}
