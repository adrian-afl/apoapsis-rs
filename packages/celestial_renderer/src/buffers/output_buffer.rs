use renderer_common::buffer_writers::write_float;
use renderer_common::errors::RenderingError;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct OutputBuffer {
    staging_buffer: VEBuffer,
    pub buffer: VEBuffer,
}

impl OutputBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<OutputBuffer, RenderingError> {
        Ok(OutputBuffer {
            staging_buffer: toolkit.create_buffer(
                &[VEBufferUsage::Uniform, VEBufferUsage::TransferSource],
                16,
                Some(VEMemoryProperties::HostCoherent),
            )?,
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Uniform, VEBufferUsage::TransferDestination],
                16,
                Some(VEMemoryProperties::DeviceLocal),
            )?,
        })
    }

    /*
    current schema:
    float exposure;
    */
    pub fn update(&mut self, exposure: f64) -> Result<(), RenderingError> {
        let ptr = self.staging_buffer.map()? as *mut f32;

        write_float(ptr, 0, exposure);

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
