use crate::ui_rendered_item::UIRenderedItem;
use glam::DVec3;
use renderer_common::buffer_writers::{
    write_bool_as_uint, write_float, write_mat4, write_vec3_zero,
};
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
                &[VEBufferUsage::Uniform],
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
    pub fn update(&mut self, item: &UIRenderedItem) -> Result<(), RenderingError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        // offset +=

        self.buffer.unmap()?;
        Ok(())
    }
}
