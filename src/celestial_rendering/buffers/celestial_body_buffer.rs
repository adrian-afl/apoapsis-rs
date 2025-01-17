use crate::celestial_rendering::buffers::buffer_writers::{
    write_float, write_mat4, write_vec3_zero,
};
use crate::celestial_rendering::errors::CelestialRendererError;
use crate::celestial_rendering::scene::camera::Camera;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferType};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct CelestialBodyBuffer {
    pub buffer: VEBuffer,
}

impl CelestialBodyBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<CelestialBodyBuffer, CelestialRendererError> {
        Ok(CelestialBodyBuffer {
            buffer: toolkit.create_buffer(
                VEBufferType::Uniform,
                8 * 1024,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:
    mat4 rotationMatrix;
    vec4 bodyCenter_zero;
    vec4 cloudsColor_zero;

    float terrainRadius;
    float waterRadius;
    float atmosphereStart;
    float cloudsMinHeight;

    float cloudsMaxHeight;
    float rayleighHeight;
    float rayleighDensity;
    float mieHeight;

    vec4 mieColor_mieDensity;

    vec4 starDirection_zero;
    vec4 starRadiance_zero;
    */
    pub fn update(&mut self, camera: &Camera, elapsed: f64) -> Result<(), CelestialRendererError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;

        offset += write_mat4(ptr, offset, camera.projection_matrix);
        offset += write_mat4(ptr, offset, camera.view_matrix);

        let frustum_cone = &camera.frustum_cone;

        offset += write_vec3_zero(ptr, offset, frustum_cone.top_left);
        offset += write_vec3_zero(ptr, offset, frustum_cone.bottom_left);
        offset += write_vec3_zero(ptr, offset, frustum_cone.top_right);
        offset += write_vec3_zero(ptr, offset, frustum_cone.bottom_right);

        offset += write_float(ptr, offset, elapsed);

        self.buffer.unmap()?;
        Ok(())
    }
}
