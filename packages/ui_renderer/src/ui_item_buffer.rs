use crate::ui_rendered_item::UIRenderedItem;
use glam::DVec3;
use renderer_common::buffer_writers::{
    write_bool_as_uint, write_float, write_mat4, write_vec3_zero,
};
use renderer_common::errors::RenderingError;
use vengine_rs::buffer::buffer::{VEBuffer, VEBufferUsage};
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;

pub struct UIItemBuffer {
    pub buffer: VEBuffer,
}

impl UIItemBuffer {
    pub fn new(toolkit: &VEToolkit) -> Result<Self, RenderingError> {
        Ok(Self {
            buffer: toolkit.create_buffer(
                &[VEBufferUsage::Uniform],
                8 * 1024,
                Some(VEMemoryProperties::HostCoherent),
            )?,
        })
    }

    /*
    current schema:
    mat4 modelMatrix;

    vec4 color_zero;

    uint useColorTexture;
    uint useRoughnessTexture;
    uint useMetalnessTexture;
    uint useEmissionTexture;

    uint useNormalTexture;
    uint useBumpTexture;
    float colorTextureScale;
    float roughnessTextureScale;

    float roughness;
    float metalness;
    float metalnessTextureScale;
    float emissionTextureScale;

    vec4 emission_zero;

    float normalTextureScale;
    float bumpTextureScale;
    */
    pub fn update(&mut self, mesh: &UIRenderedItem) -> Result<(), RenderingError> {
        let ptr = self.buffer.map()? as *mut f32;

        let mut offset = 0;
        //mat4 modelMatrix;
        // offset += write_mat4(ptr, offset, mesh.model_matrix);
        //
        // //vec4 color_zero;
        // offset += write_vec3_zero(
        //     ptr,
        //     offset,
        //     match mesh.material.color {
        //         ColorOrTexture::Color(color) => color,
        //         ColorOrTexture::Texture(_) => DVec3::ZERO,
        //     },
        // );
        //
        // /////////////////////
        //
        // //uint useColorTexture;
        // offset += write_bool_as_uint(
        //     ptr,
        //     offset,
        //     match mesh.material.color {
        //         ColorOrTexture::Color(_) => false,
        //         ColorOrTexture::Texture(_) => true,
        //     },
        // );
        //
        // //uint useRoughnessTexture;
        // offset += write_bool_as_uint(
        //     ptr,
        //     offset,
        //     match mesh.material.roughness {
        //         ValueOrTexture::Value(_) => false,
        //         ValueOrTexture::Texture(_) => true,
        //     },
        // );
        //
        // //uint useMetalnessTexture;
        // offset += write_bool_as_uint(
        //     ptr,
        //     offset,
        //     match mesh.material.metalness {
        //         ValueOrTexture::Value(_) => false,
        //         ValueOrTexture::Texture(_) => true,
        //     },
        // );
        //
        // //uint useEmissionTexture;
        // offset += write_bool_as_uint(
        //     ptr,
        //     offset,
        //     match mesh.material.emission {
        //         ColorOrTexture::Color(_) => false,
        //         ColorOrTexture::Texture(_) => true,
        //     },
        // );
        //
        // //uint useNormalTexture;
        // offset += write_bool_as_uint(
        //     ptr,
        //     offset,
        //     match mesh.material.normal {
        //         None => false,
        //         Some(_) => true,
        //     },
        // );
        //
        // //uint useBumpTexture;
        // offset += write_bool_as_uint(
        //     ptr,
        //     offset,
        //     match mesh.material.bump {
        //         None => false,
        //         Some(_) => true,
        //     },
        // );
        //
        // /////////////////////
        //
        // //float colorTextureScale;
        // offset += write_float(
        //     ptr,
        //     offset,
        //     match &mesh.material.color {
        //         ColorOrTexture::Color(_) => 1.0,
        //         ColorOrTexture::Texture(tex) => tex.scale,
        //     },
        // );
        //
        // //float roughnessTextureScale;
        // offset += write_float(
        //     ptr,
        //     offset,
        //     match &mesh.material.roughness {
        //         ValueOrTexture::Value(_) => 1.0,
        //         ValueOrTexture::Texture(tex) => tex.scale,
        //     },
        // );
        //
        // //float roughness;
        // offset += write_float(
        //     ptr,
        //     offset,
        //     match mesh.material.roughness {
        //         ValueOrTexture::Value(roughness) => roughness,
        //         ValueOrTexture::Texture(_) => 1.0,
        //     },
        // );
        //
        // //float metalness;
        // offset += write_float(
        //     ptr,
        //     offset,
        //     match mesh.material.metalness {
        //         ValueOrTexture::Value(metalness) => metalness,
        //         ValueOrTexture::Texture(_) => 1.0,
        //     },
        // );
        //
        // //float metalnessTextureScale;
        // offset += write_float(
        //     ptr,
        //     offset,
        //     match &mesh.material.metalness {
        //         ValueOrTexture::Value(_) => 1.0,
        //         ValueOrTexture::Texture(tex) => tex.scale,
        //     },
        // );
        //
        // //float emissionTextureScale;
        // offset += write_float(
        //     ptr,
        //     offset,
        //     match &mesh.material.emission {
        //         ColorOrTexture::Color(_) => 1.0,
        //         ColorOrTexture::Texture(tex) => tex.scale,
        //     },
        // );
        //
        // //vec4 emission_zero;
        // offset += write_vec3_zero(
        //     ptr,
        //     offset,
        //     match mesh.material.emission {
        //         ColorOrTexture::Color(color) => color,
        //         ColorOrTexture::Texture(_) => DVec3::ZERO,
        //     },
        // );
        //
        // //float normalTextureScale;
        // offset += write_float(
        //     ptr,
        //     offset,
        //     match &mesh.material.normal {
        //         None => 1.0,
        //         Some(tex) => tex.scale,
        //     },
        // );
        //
        // //float bumpTextureScale;
        // offset += write_float(
        //     ptr,
        //     offset,
        //     match &mesh.material.bump {
        //         None => 1.0,
        //         Some(tex) => tex.scale,
        //     },
        // );

        self.buffer.unmap()?;
        Ok(())
    }
}
