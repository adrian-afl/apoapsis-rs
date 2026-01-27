use crate::geometry::common_icosphere::{
    ICO_BASE_SUBDIVISION, calculate_base_icosphere_parts_count,
};
use ash::vk::DeviceSize;
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
        let icosphere_triangles_count =
            calculate_base_icosphere_parts_count(ICO_BASE_SUBDIVISION) as u64;
        dbg!(icosphere_triangles_count);
        // each part needs a mat4 f32, so 16 * 4 * count is total size
        // lets add some trailing space just for fun too
        let desired_buffer_size = 16 * 4 * icosphere_triangles_count + 2048;
        // let desired_buffer_size = 1024 * 1024;
        Ok(Self {
            staging_buffer: toolkit.create_buffer(
                &[VEBufferUsage::Storage, VEBufferUsage::TransferSource],
                desired_buffer_size as DeviceSize,
                Some(VEMemoryProperties::HostCoherent),
            )?,
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Storage, VEBufferUsage::TransferDestination],
                desired_buffer_size as DeviceSize,
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
