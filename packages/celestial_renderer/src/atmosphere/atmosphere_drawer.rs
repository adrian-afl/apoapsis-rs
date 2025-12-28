use crate::buffers::common_buffer::CommonBuffer;
use crate::geometry::g_buffer::GBuffer;
use ash::vk::{AccessFlags, ImageAspectFlags, ImageLayout, PipelineStageFlags};
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use std::sync::Arc;
use vengine_rs::compute::compute_stage::VEComputeStage;
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::device::VEDevice;
use vengine_rs::core::memory_barrier::{submit_barriers, VEImageMemoryBarrier};
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::filtering::VEFiltering;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;
use vengine_rs::image::sampler::{VESampler, VESamplerAddressMode};

pub struct AtmosphereDrawer {
    device: Arc<VEDevice>,

    pub compute_stage: VEComputeStage,

    pub body_data_set_layout: VEDescriptorSetLayout,

    common_data_set_layout: VEDescriptorSetLayout,
    common_data_set: VEDescriptorSet,

    pub out_additive_rgb: VEImage,
    pub out_alpha_rgba: VEImage,

    linear_sampler: VESampler,
}

static WORKGROUP_SIZE: u32 = 8; // from the shader!!! its 8x8x1

impl AtmosphereDrawer {
    pub fn new(
        config: &ResolutionConfig,
        toolkit: &VEToolkit,
        common_buffer: &CommonBuffer,
        clouds_data_low_freq: &mut VEImage,
        clouds_data_high_freq: &mut VEImage,
        g_buffer: &mut GBuffer,
    ) -> Result<AtmosphereDrawer, RenderingError> {
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

        let shader = toolkit.create_shader_module(
            "shaders/compiled/atmosphere/atmosphere.comp.spv",
            VEShaderModuleType::Compute,
        )?;

        let linear_sampler = toolkit.create_sampler(
            VESamplerAddressMode::Repeat,
            VEFiltering::Linear,
            VEFiltering::Linear,
            false,
        )?;

        let body_data_set_layout =
            toolkit.create_descriptor_set_layout(&[VEDescriptorSetLayoutField {
                // celestial data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::Compute,
            }])?;

        let mut common_data_set_layout = toolkit.create_descriptor_set_layout(&[
            VEDescriptorSetLayoutField {
                // common data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // gBufferColorRGBroughnessA image
                binding: 1,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // gBufferNormalRGBdistanceA image
                binding: 2,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // gBufferEmissionRGBmetalnessA image
                binding: 3,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // cloudsLowFreqTextureDensityR image
                binding: 4,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // cloudsHighFreqTextureDensityR image
                binding: 5,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // outAdditiveRGB image
                binding: 6,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // outAlphaRGBA image
                binding: 7,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
        ])?;

        let common_data_set = common_data_set_layout.create_descriptor_set()?;

        common_data_set.bind_buffer(0, &common_buffer.buffer)?;

        let view = g_buffer
            .color_rgb_roughness_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        common_data_set.bind_image_storage(1, &g_buffer.color_rgb_roughness_a, view)?;

        let view = g_buffer
            .normal_rgb_distance_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        common_data_set.bind_image_storage(2, &g_buffer.normal_rgb_distance_a, view)?;

        let view = g_buffer
            .emission_rgb_metalness_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        common_data_set.bind_image_storage(3, &g_buffer.emission_rgb_metalness_a, view)?;

        let view = clouds_data_low_freq.get_view(VEImageViewCreateInfo::simple_2d())?;
        common_data_set.bind_image_sampler(4, clouds_data_low_freq, view, &linear_sampler)?;

        let view = clouds_data_high_freq.get_view(VEImageViewCreateInfo::simple_3d())?;
        common_data_set.bind_image_sampler(5, clouds_data_high_freq, view, &linear_sampler)?;

        let view = out_additive_rgb.get_view(VEImageViewCreateInfo::simple_2d())?;
        common_data_set.bind_image_storage(6, &out_additive_rgb, view)?;

        let view = out_alpha_rgba.get_view(VEImageViewCreateInfo::simple_2d())?;
        common_data_set.bind_image_storage(7, &out_alpha_rgba, view)?;

        let compute_stage = toolkit
            .create_compute_stage(&[&common_data_set_layout, &body_data_set_layout], &shader)?;

        Ok(AtmosphereDrawer {
            device: toolkit.device.clone(),
            compute_stage,
            body_data_set_layout,
            common_data_set_layout,
            common_data_set,
            out_additive_rgb,
            out_alpha_rgba,
            linear_sampler,
        })
    }

    pub fn recreate_stage(&mut self, toolkit: &VEToolkit) -> Result<(), RenderingError> {
        let shader = toolkit.create_shader_module(
            "shaders/compiled/atmosphere/atmosphere.comp.spv",
            VEShaderModuleType::Compute,
        )?;

        self.compute_stage = toolkit.create_compute_stage(
            &[&self.common_data_set_layout, &self.body_data_set_layout],
            &shader,
        )?;

        Ok(())
    }

    pub fn record(
        &self,
        command_buffer: &VECommandBuffer,
        body_data_set: &VEDescriptorSet,
        config: &ResolutionConfig,
    ) {
        self.compute_stage.bind(command_buffer);

        self.compute_stage
            .set_descriptor_set(command_buffer, 0, &self.common_data_set);

        self.compute_stage
            .set_descriptor_set(command_buffer, 1, body_data_set);

        self.compute_stage.dispatch(
            command_buffer,
            config.width / WORKGROUP_SIZE,
            config.height / WORKGROUP_SIZE,
            1,
        );

        let barrier_additive = VEImageMemoryBarrier {
            image: self.out_additive_rgb.handle,
            aspect: ImageAspectFlags::COLOR,
            src_access: AccessFlags::SHADER_WRITE,
            dst_access: AccessFlags::SHADER_READ,
            old_layout: ImageLayout::GENERAL,
            new_layout: ImageLayout::GENERAL,
        };

        let barrier_alpha = VEImageMemoryBarrier {
            image: self.out_alpha_rgba.handle,
            aspect: ImageAspectFlags::COLOR,
            src_access: AccessFlags::SHADER_WRITE,
            dst_access: AccessFlags::SHADER_READ,
            old_layout: ImageLayout::GENERAL,
            new_layout: ImageLayout::GENERAL,
        };

        submit_barriers(
            &self.device,
            command_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::ALL_COMMANDS,
            &[],
            &[],
            &[barrier_additive.build(), barrier_alpha.build()],
        );
    }
}
