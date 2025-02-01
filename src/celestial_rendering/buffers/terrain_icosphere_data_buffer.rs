use crate::celestial_rendering::buffers::buffer_writers::{
    write_float, write_mat4, write_vec3_zero,
};
use crate::celestial_rendering::errors::RenderingError;
use glam::DMat4;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct TerrainIcosphereDataBuffer {
    pub buffer: VEBuffer,
}

impl TerrainIcosphereDataBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<TerrainIcosphereDataBuffer, RenderingError> {
        Ok(TerrainIcosphereDataBuffer {
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Storage],
                64 * 1024,
                Some(VEMemoryProperties::HostCoherent), // should REALLY be device local...
            )?,
        })
    }

    /*
    current schema:
    mat4 partMatrix[320];
    */
    pub fn update(&mut self, part_matrices: &[DMat4]) -> Result<(), RenderingError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        for matrix in part_matrices {
            offset += write_mat4(ptr, offset, *matrix);
        }

        self.buffer.unmap()?;
        Ok(())
    }
}
