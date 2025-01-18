use crate::celestial_rendering::buffers::celestial_body_buffer::CelestialBodyBuffer;
use crate::celestial_rendering::buffers::cloud_generator_low_freq_buffer::CloudGeneratorLowFreqBuffer;
use crate::celestial_rendering::buffers::common_buffer::CommonBuffer;
use crate::celestial_rendering::errors::CelestialRendererError;
use crate::celestial_rendering::geometry::g_buffer::GBuffer;
use crate::config::Config;
use vengine_rs::compute::compute_stage::VEComputeStage;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::filtering::VEFiltering;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;
use vengine_rs::image::sampler::VESamplerAddressMode;

pub struct AtmosphereDrawer {
    pub compute_stage: VEComputeStage,

    data_set_layout: VEDescriptorSetLayout,
    data_set: VEDescriptorSet,

    pub out_additive_rgb: VEImage,
    pub out_alpha_rgba: VEImage,
}

static WORKGROUP_SIZE: u32 = 8; // from the shader!!! its 8x8x1

impl AtmosphereDrawer {
    pub fn new(
        config: &Config,
        toolkit: &VEToolkit,
        common_buffer: &CommonBuffer,
        clouds_data_low_freq: &mut VEImage,
        clouds_data_high_freq: &mut VEImage,
        g_buffer: &mut GBuffer,
    ) -> Result<AtmosphereDrawer, CelestialRendererError> {
        let mut out_additive_rgb = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA16f,
            &[VEImageUsage::Storage, VEImageUsage::Sampled],
        )?;

        let mut out_alpha_rgba = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA16f,
            &[VEImageUsage::Storage, VEImageUsage::Sampled],
        )?;

        let low_freq_compute_shader = toolkit.create_shader_module(
            "shaders/compiled/atmosphere/atmosphere.comp.spv",
            VEShaderModuleType::Compute,
        )?;

        let nearest_sampler = toolkit.create_sampler(
            VESamplerAddressMode::Repeat,
            VEFiltering::Nearest,
            VEFiltering::Nearest,
            false,
        )?;

        let linear_sampler = toolkit.create_sampler(
            VESamplerAddressMode::Repeat,
            VEFiltering::Linear,
            VEFiltering::Linear,
            false,
        )?;

        let mut data_set_layout = toolkit.create_descriptor_set_layout(&[
            VEDescriptorSetLayoutField {
                // common data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // celestial data buffer
                binding: 1,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // gBufferColorRGBroughnessA image
                binding: 2,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // gBufferNormalRGBdistanceA image
                binding: 3,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // gBufferEmissionRGBmetalnessA image
                binding: 4,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // cloudsLowFreqTextureDensityR image
                binding: 5,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // cloudsHighFreqTextureDensityR image
                binding: 6,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // outAdditiveRGB image
                binding: 7,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // outAlphaRGBA image
                binding: 8,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
        ])?;

        let data_set = data_set_layout.create_descriptor_set()?;

        data_set.bind_buffer(0, &common_buffer.buffer)?;

        let view = g_buffer
            .color_rgb_roughness_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_sampler(2, &g_buffer.color_rgb_roughness_a, view, &nearest_sampler)?;

        let view = g_buffer
            .normal_rgb_distance_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_sampler(3, &g_buffer.normal_rgb_distance_a, view, &nearest_sampler)?;

        let view = g_buffer
            .emission_rgb_metalness_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_sampler(
            4,
            &g_buffer.emission_rgb_metalness_a,
            view,
            &nearest_sampler,
        )?;

        let view = clouds_data_low_freq.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_sampler(5, clouds_data_low_freq, view, &linear_sampler)?;

        let view = clouds_data_high_freq.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_sampler(6, clouds_data_high_freq, view, &linear_sampler)?;

        let view = out_additive_rgb.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(7, &out_additive_rgb, view)?;

        let view = out_alpha_rgba.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(8, &out_alpha_rgba, view)?;

        let compute_stage =
            toolkit.create_compute_stage(&[&data_set_layout], &low_freq_compute_shader)?;

        compute_stage.begin_recording()?;
        compute_stage.set_descriptor_set(0, &data_set);
        compute_stage.dispatch(
            config.width / WORKGROUP_SIZE,
            config.height / WORKGROUP_SIZE,
            1,
        );
        compute_stage.end_recording()?;

        Ok(AtmosphereDrawer {
            compute_stage,
            data_set_layout,
            data_set,
            out_additive_rgb,
            out_alpha_rgba,
        })
    }

    pub fn set_celestial_buffer(
        &self,
        celestial_buffer: &CelestialBodyBuffer,
    ) -> Result<(), CelestialRendererError> {
        self.data_set.bind_buffer(1, &celestial_buffer.buffer)?;
        Ok(())
    }
}
