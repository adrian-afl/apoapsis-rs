use crate::celestial_rendering::buffers::buffer_writers::{
    write_float, write_mat4, write_vec3_zero,
};
use crate::celestial_rendering::errors::CelestialRendererError;
use glam::{DMat4, DVec3};
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferType};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct WaterIcosphereDataBuffer {
    pub buffer: VEBuffer,
}

impl WaterIcosphereDataBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<WaterIcosphereDataBuffer, CelestialRendererError> {
        Ok(WaterIcosphereDataBuffer {
            buffer: toolkit.create_buffer(
                VEBufferType::Storage,
                64 * 1024,
                Some(VEMemoryProperties::HostCoherent), // should REALLY be device local...
            )?,
        })
    }

    /*
    current schema:
    vec4 waterColor_zero;
    vec4 bodyCenter_zero;
    mat4 partMatrix[320];
    */
    pub fn update(
        &mut self,
        water_color: DVec3,
        body_center_camera_space: DVec3,
        part_matrices: &[DMat4],
    ) -> Result<(), CelestialRendererError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        offset += write_vec3_zero(ptr, offset, water_color);
        offset += write_vec3_zero(ptr, offset, body_center_camera_space);

        for matrix in part_matrices {
            offset += write_mat4(ptr, offset, *matrix);
        }

        self.buffer.unmap()?;
        Ok(())
    }
}
