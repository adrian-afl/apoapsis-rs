use crate::celestial_rendering::errors::CelestialRendererError;
use crate::config::Config;
use vengine_rs::core::memory_properties::VEMemoryProperties;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::{VEImage, VEImageUsage};
use vengine_rs::image::image_format::VEImageFormat;

pub struct GBuffer {
    pub color_rgb_roughness_a: VEImage,
    pub emission_rgb_metalness_a: VEImage,
    pub normal_rgb_distance_a: VEImage,
    pub shared_depth_buffer: VEImage,
}

impl GBuffer {
    pub fn new(config: &Config, toolkit: &VEToolkit) -> Result<GBuffer, CelestialRendererError> {
        let color_rgb_roughness_a = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA8unorm,
            &[VEImageUsage::ColorAttachment, VEImageUsage::Storage],
        )?;

        let emission_rgb_metalness_a = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA8unorm,
            &[VEImageUsage::ColorAttachment, VEImageUsage::Storage],
        )?;

        let normal_rgb_distance_a = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA32f,
            &[VEImageUsage::ColorAttachment, VEImageUsage::Storage],
        )?;

        let shared_depth_buffer = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::Depth32f,
            &[VEImageUsage::DepthAttachment, VEImageUsage::Sampled],
        )?;

        //##########

        Ok(GBuffer {
            color_rgb_roughness_a,
            emission_rgb_metalness_a,
            normal_rgb_distance_a,
            shared_depth_buffer,
        })
    }
}
