use glam::DMat4;
use renderer_common::buffer_writers::write_mat4;
use renderer_common::errors::RenderingError;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct TerrainIcosphereDataBuffer {
    staging_buffer: VEBuffer,
    pub buffer: VEBuffer,
}

impl TerrainIcosphereDataBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<Self, RenderingError> {
        Ok(Self {
            staging_buffer: toolkit.create_buffer(
                &[VEBufferUsage::Storage, VEBufferUsage::TransferSource],
                64 * 1024,
                Some(VEMemoryProperties::HostCoherent),
            )?,
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Storage, VEBufferUsage::TransferDestination],
                64 * 1024,
                Some(VEMemoryProperties::DeviceLocal),
            )?,
        })
    }

    /*
    current schema:
    mat4 partMatrix[320];
    */
    pub fn update(&mut self, part_matrices: &[DMat4]) -> Result<(), RenderingError> {
        let ptr = self.staging_buffer.map()? as *mut f32;

        let mut offset = 0;

        for matrix in part_matrices {
            offset += write_mat4(ptr, offset, *matrix);
        }

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
