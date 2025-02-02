use renderer_common::buffer_writers::{write_float, write_mat4, write_vec3_zero};
use renderer_common::errors::RenderingError;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct UICommonBuffer {
    pub buffer: VEBuffer,
}

impl UICommonBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<Self, RenderingError> {
        Ok(Self {
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Uniform],
                512,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:
    mat4 perspectiveMatrix;
    mat4 viewMatrix;

    vec4 frustumTopLeft_zero;
    vec4 frustumBottomLeft_zero;
    vec4 frustumTopRight_zero;
    vec4 frustumBottomRight_zero;

    vec4 elapsed_zero_zero_zero;
    */
    pub fn update(&mut self) -> Result<(), RenderingError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;
        //
        // offset += write_mat4(ptr, offset, camera.projection_matrix);
        // offset += write_mat4(ptr, offset, camera.view_matrix);
        //
        // let frustum_cone = &camera.frustum_cone;
        //
        // offset += write_vec3_zero(ptr, offset, frustum_cone.top_left);
        // offset += write_vec3_zero(ptr, offset, frustum_cone.bottom_left);
        // offset += write_vec3_zero(ptr, offset, frustum_cone.top_right);
        // offset += write_vec3_zero(ptr, offset, frustum_cone.bottom_right);
        //
        // offset += write_float(ptr, offset, elapsed);

        self.buffer.unmap()?;
        Ok(())
    }
}
