use crate::font_atlas_generator::font_atlas_generator::FontAtlas;
use crate::ui_item_buffer::UIItemBuffer;
use ecs::components::ui::ui_text_component::UIFontSize;
use glam::{DVec2, DVec4};
use renderer_common::errors::RenderingError;
use std::sync::Mutex;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayout;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::filtering::VEFiltering;
use vengine_rs::image::image::VEImage;
use vengine_rs::image::sampler::{VESampler, VESamplerAddressMode};

pub struct UIRenderedItem {
    pub size: DVec2,
    pub position: DVec2,
    pub orientation: f64, // radians
    pub z_index: i32,

    pub color: DVec4,
    pub texture: Option<VEImage>,
    pub texture_component_id: Option<u64>,

    pub text_color: DVec4,
    pub text: String,
    pub font_size: UIFontSize,

    pub descriptor_set: VEDescriptorSet,
    sampler: VESampler,
    pub item_buffer: Mutex<UIItemBuffer>,
}

impl UIRenderedItem {
    pub fn empty(
        toolkit: &VEToolkit,
        layout: &mut VEDescriptorSetLayout,
    ) -> Result<Self, RenderingError> {
        Ok(Self {
            size: DVec2::new(0.0, 0.0),
            position: DVec2::new(0.0, 0.0),
            orientation: 0.0,
            z_index: 0,
            color: DVec4::new(0.0, 0.0, 0.0, 0.0),
            texture: None,
            texture_component_id: None,

            text_color: DVec4::new(0.0, 0.0, 0.0, 0.0),
            text: "".to_owned(),
            font_size: UIFontSize::Medium,

            descriptor_set: layout.create_descriptor_set()?,
            item_buffer: Mutex::from(UIItemBuffer::new(toolkit)?),
            sampler: toolkit.create_sampler(
                VESamplerAddressMode::Repeat,
                VEFiltering::Linear,
                VEFiltering::Linear,
                true,
            )?,
        })
    }

    pub fn update_buffer(
        &self,
        font_atlas_small: &FontAtlas,
        font_atlas_medium: &FontAtlas,
        font_atlas_large: &FontAtlas,
    ) -> Result<(), RenderingError> {
        self.item_buffer.lock().unwrap().update(
            self,
            font_atlas_small,
            font_atlas_medium,
            font_atlas_large,
        )
    }
}
