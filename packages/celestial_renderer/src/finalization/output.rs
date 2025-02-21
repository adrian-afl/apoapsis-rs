use crate::buffers::output_buffer::OutputBuffer;
use crate::finalization::multi_merger::MultiMerger;
use ash::vk;
use ash::vk::{AccessFlags, ImageAspectFlags, ImageLayout, PipelineStageFlags};
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use std::sync::Arc;
use ui_renderer::ui_drawer::UIDrawer;
use vengine_rs::compute::compute_stage::VEComputeStage;
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::device::VEDevice;
use vengine_rs::core::memory_barrier::{submit_barriers, VEImageMemoryBarrier, VEMemoryBarrier};
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;

pub struct Output {
    config: ResolutionConfig,

    device: Arc<VEDevice>,

    pub compute_stage: VEComputeStage,

    data_set_layout: VEDescriptorSetLayout,
    data_set: VEDescriptorSet,

    buffer: OutputBuffer,

    pub output: VEImage,
}

static WORKGROUP_SIZE: u32 = 8; // from the shader!!! its 8x8x1

impl Output {
    pub fn new(
        config: &ResolutionConfig,
        multi_merger: &mut MultiMerger,
        ui_drawer: &mut UIDrawer,
        toolkit: &VEToolkit,
    ) -> Result<Output, RenderingError> {
        let mut output = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA16f,
            &[
                VEImageUsage::Storage,
                VEImageUsage::Sampled,
                VEImageUsage::TransferSource,
            ],
        )?;

        let shader = toolkit.create_shader_module(
            "shaders/compiled/output/output.comp.spv",
            VEShaderModuleType::Compute,
        )?;

        let buffer = OutputBuffer::new(toolkit)?;

        let mut data_set_layout = toolkit.create_descriptor_set_layout(&[
            VEDescriptorSetLayoutField {
                // output input buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // celestialResultTexture input
                binding: 1,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // ui result
                binding: 2,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // output image
                binding: 3,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
        ])?;

        let data_set = data_set_layout.create_descriptor_set()?;

        data_set.bind_buffer(0, &buffer.buffer)?;

        let view = multi_merger
            .output
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(1, &multi_merger.output, view)?;

        let view = ui_drawer // TODO hmmmmmm this could be a nice macro
            .out_color
            .get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(2, &ui_drawer.out_color, view)?;

        let view = output.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(3, &output, view)?;

        let compute_stage = toolkit.create_compute_stage(&[&data_set_layout], &shader)?;

        Ok(Output {
            device: toolkit.device.clone(),
            compute_stage,
            data_set_layout,
            data_set,
            output,
            buffer,
            config: config.clone(),
        })
    }

    pub fn record(&self, command_buffer: &VECommandBuffer, multi_merger: &MultiMerger) {
        self.compute_stage.bind(&command_buffer);
        self.compute_stage
            .set_descriptor_set(&command_buffer, 0, &self.data_set);
        self.compute_stage.dispatch(
            &command_buffer,
            self.config.width / WORKGROUP_SIZE,
            self.config.height / WORKGROUP_SIZE,
            1,
        );

        let barrier = VEImageMemoryBarrier {
            image: multi_merger.output.handle,
            aspect: ImageAspectFlags::COLOR,
            src_access: AccessFlags::SHADER_READ,
            dst_access: AccessFlags::SHADER_WRITE
                | AccessFlags::MEMORY_WRITE
                | AccessFlags::TRANSFER_WRITE,
            old_layout: ImageLayout::GENERAL,
            new_layout: ImageLayout::GENERAL,
        };

        submit_barriers(
            &self.device,
            &command_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::ALL_COMMANDS,
            &[],
            &[],
            &[barrier.build()],
        );
        //
        // // this here is unhinged
        // // its to clear the multimerger
        unsafe {
            self.device.device.cmd_clear_color_image(
                command_buffer.handle,
                multi_merger.output.handle,
                multi_merger.output.current_layout,
                &vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
                &[vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1) // TODO mip mapping
                    .base_array_layer(0)
                    .layer_count(1)],
            )
        }

        let barrier = VEImageMemoryBarrier {
            image: multi_merger.output.handle,
            aspect: ImageAspectFlags::COLOR,
            src_access: AccessFlags::SHADER_WRITE
                | AccessFlags::MEMORY_WRITE
                | AccessFlags::TRANSFER_WRITE,
            dst_access: AccessFlags::SHADER_READ,
            old_layout: ImageLayout::GENERAL,
            new_layout: ImageLayout::GENERAL,
        };

        submit_barriers(
            &self.device,
            &command_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::ALL_COMMANDS,
            &[],
            &[],
            &[barrier.build()],
        );
    }

    pub fn recreate_stage(
        &mut self,
        toolkit: &VEToolkit,
        multi_merger: &mut MultiMerger,
    ) -> Result<(), RenderingError> {
        let shader = toolkit.create_shader_module(
            "shaders/compiled/output/output.comp.spv",
            VEShaderModuleType::Compute,
        )?;

        self.compute_stage = toolkit.create_compute_stage(&[&self.data_set_layout], &shader)?;

        Ok(())
    }

    pub fn update_buffer(&mut self, exposure: f64) -> Result<(), RenderingError> {
        self.buffer.update(exposure)?;
        Ok(())
    }
}
