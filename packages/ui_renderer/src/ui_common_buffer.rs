use crate::font_atlas_generator::common::{CharPositionArrayItem, FontAtlas};
use renderer_common::buffer_writers::{write_float, write_mat4, write_vec2, write_vec3_zero};
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct UICommonBuffer {
    pub buffer: VEBuffer,
}

impl UICommonBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<Self, RenderingError> {
        Ok(Self {
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Storage],
                16 * 1024,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:

    struct FontAtlasIndices {
        float x;
        float y;
        float w;
        float h;
    };

    vec4 resolution_zero_zero;
    FontAtlasIndices fontAtlasSmallData[255];
    FontAtlasIndices fontAtlasMediumData[255];
    FontAtlasIndices fontAtlasLargeData[255];
    */
    pub fn update(
        &mut self,
        config: &ResolutionConfig,
        font_atlas_small: &FontAtlas,
        font_atlas_medium: &FontAtlas,
        font_atlas_large: &FontAtlas,
    ) -> Result<(), RenderingError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        // vec4 resolution_zero_zero;
        offset += write_float(ptr, offset, config.width as f64);
        offset += write_float(ptr, offset, config.height as f64);
        offset += write_float(ptr, offset, 0.0);
        offset += write_float(ptr, offset, 0.0);

        let atlases = [font_atlas_small, font_atlas_medium, font_atlas_large];

        for atlas in atlases {
            for i in 0..255 {
                let c = atlas.letters_array.get(i);
                let c = c.unwrap_or_else(|| &CharPositionArrayItem {
                    c: ' ', // in vec4:
                    x: 0,   // x
                    y: 0,   // y
                    w: 0,   // z
                    h: 0,   // w
                });
                // println!("for index {i} eek is {:?}", c);
                offset += write_float(ptr, offset, c.x as f64);
                offset += write_float(ptr, offset, c.y as f64);
                offset += write_float(ptr, offset, c.w as f64);
                offset += write_float(ptr, offset, c.h as f64);
            }
        }

        Ok(())
    }
}
