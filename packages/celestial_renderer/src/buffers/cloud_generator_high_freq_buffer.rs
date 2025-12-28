use glam::{DVec3, DVec4};
use planet_generator_library::noise::fbm;
use renderer_common::buffer_writers::{write_float, write_vec4};
use renderer_common::errors::RenderingError;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct CloudGeneratorHighFreqBuffer {
    staging_buffer: VEBuffer,
    pub buffer: VEBuffer,
}

impl CloudGeneratorHighFreqBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<CloudGeneratorHighFreqBuffer, RenderingError> {
        Ok(CloudGeneratorHighFreqBuffer {
            staging_buffer: toolkit.create_buffer(
                &[VEBufferUsage::Uniform, VEBufferUsage::TransferSource],
                128,
                Some(VEMemoryProperties::HostCoherent),
            )?,
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Uniform, VEBufferUsage::TransferDestination],
                128,
                Some(VEMemoryProperties::DeviceLocal),
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
    ) -> Result<(), RenderingError> {
        let ptr = self.staging_buffer.map()? as *mut f32;

        let mut offset = 0;

        offset += write_vec4(ptr, offset, seed);
        offset += write_float(
            ptr,
            offset,
            (fbm(DVec3::new(elapsed, seed.x, seed.y), 6, 2.0, 0.5) * 2.0 - 1.0) * 100.0,
        );
        write_float(ptr, offset, frequency);

        Ok(())
    }

    pub fn record_copy_from_staging(&self, command_buffer: &VECommandBuffer) {
        self.staging_buffer.copy_to_cmd(
            command_buffer,
            &self.buffer,
            0,
            0,
            self.staging_buffer.size,
        );
    }
}
