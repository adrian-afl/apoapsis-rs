use crate::celestial_rendering::buffers::buffer_writers::{write_float, write_vec4};
use crate::celestial_rendering::errors::CelestialRendererError;
use glam::DVec4;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct CloudGeneratorLowFreqBuffer {
    pub buffer: VEBuffer,
}

impl CloudGeneratorLowFreqBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<CloudGeneratorLowFreqBuffer, CelestialRendererError> {
        Ok(CloudGeneratorLowFreqBuffer {
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Uniform],
                128,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:
    vec4 seed;
    vec4 elapsed_frequency_zero_zero;
    */
    pub fn update(
        &mut self,
        seed: DVec4,
        elapsed: f64,
        frequency: f64,
    ) -> Result<(), CelestialRendererError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        offset += write_vec4(ptr, offset, seed);
        offset += write_float(ptr, offset, elapsed);
        offset += write_float(ptr, offset, frequency);

        self.buffer.unmap()?;
        Ok(())
    }
}
