use crate::celestial_rendering::buffers::cloud_generator_high_freq_buffer::CloudGeneratorHighFreqBuffer;
use crate::celestial_rendering::buffers::output_buffer::OutputBuffer;
use crate::celestial_rendering::errors::CelestialRendererError;
use crate::celestial_rendering::finalization::multi_merger::MultiMerger;
use crate::config::Config;
use ash::vk;
use glam::DVec4;
use tracing::{event, Level};
use vengine_rs::compute::compute_stage::VEComputeStage;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::helpers::clear_color_f32;
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::filtering::VEFiltering;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;
use vengine_rs::image::sampler::{VESampler, VESamplerAddressMode};

pub struct Output {
    pub compute_stage: VEComputeStage,

    data_set_layout: VEDescriptorSetLayout,
    data_set: VEDescriptorSet,

    buffer: OutputBuffer,

    pub output: VEImage,
}

static WORKGROUP_SIZE: u32 = 8; // from the shader!!! its 8x8x1

impl Output {
    pub fn new(
        config: &Config,
        multi_merger: &mut MultiMerger,
        toolkit: &VEToolkit,
    ) -> Result<Output, CelestialRendererError> {
        event!(Level::WARN, "Creating Output");
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

        let hi_freq_compute_shader = toolkit.create_shader_module(
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
                // output image
                binding: 2,
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

        let view = output.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(2, &output, view)?;

        let compute_stage =
            toolkit.create_compute_stage(&[&data_set_layout], &hi_freq_compute_shader)?;

        compute_stage.begin_recording()?;
        compute_stage.set_descriptor_set(0, &data_set);
        compute_stage.dispatch(
            config.width / WORKGROUP_SIZE,
            config.height / WORKGROUP_SIZE,
            1,
        );

        // this here is unhinged
        // its to clear the multimerger
        unsafe {
            toolkit.device.device.cmd_clear_color_image(
                compute_stage.command_buffer.handle,
                multi_merger.output.handle,
                multi_merger.output.current_layout,
                &vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
                &[vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1) // TODO mip mapping
                    .base_array_layer(0)
                    .layer_count(1)],
            )
        }
        compute_stage.end_recording()?;

        Ok(Output {
            compute_stage,
            data_set_layout,
            data_set,
            output,
            buffer,
        })
    }

    pub fn update_buffer(&mut self, exposure: f64) -> Result<(), CelestialRendererError> {
        self.buffer.update(exposure)?;
        Ok(())
    }
}
