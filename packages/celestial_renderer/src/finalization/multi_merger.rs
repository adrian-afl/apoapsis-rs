use ash::vk::{AccessFlags, ImageAspectFlags, ImageLayout, PipelineStageFlags};
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
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
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;

pub struct MultiMerger {
    pub compute_stage: VEComputeStage,

    data_set_layout: VEDescriptorSetLayout,
    data_set: VEDescriptorSet,

    pub output: VEImage,
}

static WORKGROUP_SIZE: u32 = 8; // from the shader!!! its 8x8x1

impl MultiMerger {
    pub fn new(
        config: &ResolutionConfig,
        toolkit: &VEToolkit,
        input_additive: &mut VEImage,
        input_alpha: &mut VEImage,
    ) -> Result<MultiMerger, RenderingError> {
        let mut output = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA16f,
            &[
                VEImageUsage::Storage,
                VEImageUsage::Sampled,
                VEImageUsage::TransferDestination,
            ],
        )?;

        let hi_freq_compute_shader = toolkit.create_shader_module(
            "shaders/compiled/output/multi-merger.comp.spv",
            VEShaderModuleType::Compute,
        )?;

        let mut data_set_layout = toolkit.create_descriptor_set_layout(&[
            VEDescriptorSetLayoutField {
                // additiveRGB input
                binding: 0,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // alphaRGBA input
                binding: 1,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // storage image
                binding: 2,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
        ])?;

        let data_set = data_set_layout.create_descriptor_set()?;

        let view = output.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(2, &output, view)?;

        let compute_stage =
            toolkit.create_compute_stage(&[&data_set_layout], &hi_freq_compute_shader)?;

        let view = input_additive.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(0, input_additive, view)?;

        let view = input_alpha.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(1, input_alpha, view)?;

        Ok(MultiMerger {
            compute_stage,
            data_set_layout,
            data_set,
            output,
        })
    }

    pub fn recreate_stage(&mut self, toolkit: &VEToolkit) -> Result<(), RenderingError> {
        let shader = toolkit.create_shader_module(
            "shaders/compiled/output/multi-merger.comp.spv",
            VEShaderModuleType::Compute,
        )?;

        self.compute_stage = toolkit.create_compute_stage(&[&self.data_set_layout], &shader)?;

        Ok(())
    }

    pub fn record(
        &self,
        device: &VEDevice,
        command_buffer: &VECommandBuffer,
        config: &ResolutionConfig,
    ) {
        self.compute_stage.bind(command_buffer);
        self.compute_stage
            .set_descriptor_set(command_buffer, 0, &self.data_set);
        self.compute_stage.dispatch(
            command_buffer,
            config.width / WORKGROUP_SIZE,
            config.height / WORKGROUP_SIZE,
            1,
        );

        let barrier = VEImageMemoryBarrier {
            image: self.output.handle,
            aspect: ImageAspectFlags::COLOR,
            src_access: AccessFlags::SHADER_WRITE,
            dst_access: AccessFlags::SHADER_READ,
            old_layout: ImageLayout::GENERAL,
            new_layout: ImageLayout::GENERAL,
        };

        submit_barriers(
            device,
            command_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::COMPUTE_SHADER,
            &[],
            &[],
            &[barrier.build()],
        );
    }
}
