use glam::DVec4;
use renderer_common::buffer_writers::{write_float, write_int, write_vec4};
use renderer_common::errors::RenderingError;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct OutputBuffer {
    pub buffer: VEBuffer,
}

impl OutputBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<OutputBuffer, RenderingError> {
        Ok(OutputBuffer {
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Uniform],
                16,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:
    float exposure;
    */
    pub fn update(&mut self, exposure: f64) -> Result<(), RenderingError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        offset += write_float(ptr, offset, exposure);

        Ok(())
    }
}
