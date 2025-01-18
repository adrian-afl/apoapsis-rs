use crate::celestial_rendering::buffers::cloud_generator_low_freq_buffer::CloudGeneratorLowFreqBuffer;
use crate::celestial_rendering::errors::CelestialRendererError;
use glam::DVec4;
use vengine_rs::compute::compute_stage::VEComputeStage;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::{VEImage, VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;

pub struct CloudGeneratorLowFreq {
    pub compute_stage: VEComputeStage,

    data_set_layout: VEDescriptorSetLayout,
    data_set: VEDescriptorSet,

    buffer: CloudGeneratorLowFreqBuffer,

    pub low_freq_data_r: VEImage, // I could... make this a cubemap and fill it with a compute shader
}

static LOW_FREQ_RES_WIDTH: u32 = 2048;
static LOW_FREQ_RES_HEIGHT: u32 = 1024;
static WORKGROUP_SIZE: u32 = 8; // from the shader!!! its 8x8x1

impl CloudGeneratorLowFreq {
    pub fn new(toolkit: &VEToolkit) -> Result<CloudGeneratorLowFreq, CelestialRendererError> {
        let mut low_freq_data_r = toolkit.create_image_full(
            LOW_FREQ_RES_WIDTH,
            LOW_FREQ_RES_HEIGHT,
            1,
            VEImageFormat::R16f,
            &[VEImageUsage::Storage, VEImageUsage::Sampled],
        )?;

        let low_freq_compute_shader = toolkit.create_shader_module(
            "shaders/compiled/atmosphere/gen-clouds.comp.spv",
            VEShaderModuleType::Compute,
        )?;

        let buffer = CloudGeneratorLowFreqBuffer::new(toolkit)?;

        let mut data_set_layout = toolkit.create_descriptor_set_layout(&[
            VEDescriptorSetLayoutField {
                // data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::Compute,
            },
            VEDescriptorSetLayoutField {
                // storage image
                binding: 1,
                typ: VEDescriptorSetFieldType::StorageImage,
                stage: VEDescriptorSetFieldStage::Compute,
            },
        ])?;

        let data_set = data_set_layout.create_descriptor_set()?;

        data_set.bind_buffer(0, &buffer.buffer)?;
        let view = low_freq_data_r.get_view(VEImageViewCreateInfo::simple_2d())?;
        data_set.bind_image_storage(1, &low_freq_data_r, view)?;

        let compute_stage =
            toolkit.create_compute_stage(&[&data_set_layout], &low_freq_compute_shader)?;

        compute_stage.begin_recording()?;
        compute_stage.set_descriptor_set(0, &data_set);
        compute_stage.dispatch(
            LOW_FREQ_RES_WIDTH / WORKGROUP_SIZE,
            LOW_FREQ_RES_HEIGHT / WORKGROUP_SIZE,
            1,
        );
        compute_stage.end_recording()?;

        Ok(CloudGeneratorLowFreq {
            compute_stage,
            data_set_layout,
            data_set,
            buffer,
            low_freq_data_r,
        })
    }

    pub fn update_buffer(
        &mut self,
        seed: DVec4,
        elapsed: f64,
    ) -> Result<(), CelestialRendererError> {
        self.buffer.update(seed, elapsed)?;
        Ok(())
    }
}
