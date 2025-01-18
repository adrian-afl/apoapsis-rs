use crate::celestial_rendering::buffers::buffer_writers::{write_float, write_int, write_vec4};
use crate::celestial_rendering::errors::CelestialRendererError;
use glam::DVec4;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferType};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct OutputBuffer {
    pub buffer: VEBuffer,
}

impl OutputBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<OutputBuffer, CelestialRendererError> {
        Ok(OutputBuffer {
            buffer: toolkit.create_buffer(
                VEBufferType::Uniform,
                128,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:
    float exposure;
    int debugTextureIndex;
    */
    pub fn update(
        &mut self,
        exposure: f64,
        debug_texture_index: u8,
    ) -> Result<(), CelestialRendererError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        offset += write_float(ptr, offset, exposure);
        offset += write_int(ptr, offset, debug_texture_index as i32);

        self.buffer.unmap()?;
        Ok(())
    }
}
