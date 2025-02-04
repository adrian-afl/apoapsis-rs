use crate::font_atlas_generator::font_atlas_generator::FontAtlas;
use crate::ui_rendered_item::UIRenderedItem;
use ecs::components::ui::ui_text_component::UIFontSize;
use renderer_common::buffer_writers::{write_float, write_uint, write_vec2, write_vec4};
use renderer_common::errors::RenderingError;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct UIItemBuffer {
    pub buffer: VEBuffer,
}

impl UIItemBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<Self, RenderingError> {
        Ok(Self {
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Storage],
                8 * 1024,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:

    vec4 size_position;
    vec4 orientation_zero_zero;
    vec4 color;
    vec4 text_color;
    uvec4 useTexture_textLength_textFontSize;
    uint text[1024];

    */
    pub fn update(
        &mut self,
        item: &UIRenderedItem,
        font_atlas_small: &FontAtlas,
        font_atlas_medium: &FontAtlas,
        font_atlas_large: &FontAtlas,
    ) -> Result<(), RenderingError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        // vec4 size_position;
        offset += write_vec2(ptr, offset, item.size);
        offset += write_vec2(ptr, offset, item.position);

        // vec4 orientation_zero_zero;
        offset += write_float(ptr, offset, item.orientation);
        offset += write_float(ptr, offset, 0.0);
        offset += write_float(ptr, offset, 0.0);
        offset += write_float(ptr, offset, 0.0);

        // vec4 color;
        offset += write_vec4(ptr, offset, item.color);

        //vec4 text_color;
        offset += write_vec4(ptr, offset, item.text_color);

        // uvec4 useTexture_textLength_textFontSize;
        offset += write_uint(ptr, offset, if item.texture.is_some() { 1 } else { 0 });
        offset += write_uint(ptr, offset, item.text.len() as u32);
        offset += write_uint(
            ptr,
            offset,
            match item.font_size {
                UIFontSize::Small => 1,
                UIFontSize::Medium => 2,
                UIFontSize::Large => 3,
            },
        );
        offset += write_uint(ptr, offset, 0);

        let atlas = match item.font_size {
            UIFontSize::Small => font_atlas_small,
            UIFontSize::Medium => font_atlas_medium,
            UIFontSize::Large => font_atlas_large,
        };

        for char in item.text.chars() {
            let index = atlas.letters_indices.get(&char);
            let index = match index {
                None => 0,
                Some(index) => *index,
            };
            offset += write_uint(ptr, offset, index as u32);
        }

        self.buffer.unmap()?;
        Ok(())
    }
}
