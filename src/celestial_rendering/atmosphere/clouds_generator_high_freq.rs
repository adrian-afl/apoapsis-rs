use crate::celestial_rendering::buffers::cloud_generator_high_freq_buffer::CloudGeneratorHighFreqBuffer;
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

pub struct CloudGeneratorHighFreq {
    pub compute_stage: VEComputeStage,

    data_set_layout: VEDescriptorSetLayout,
    data_set: VEDescriptorSet,

    buffer: CloudGeneratorHighFreqBuffer,

    pub high_freq_data_r: VEImage,
}

static HI_FREQ_RES: u32 = 64;
static WORKGROUP_SIZE: u32 = 4; // from the shader!!! its 4x4x4

impl CloudGeneratorHighFreq {
    pub fn new(toolkit: &VEToolkit) -> Result<CloudGeneratorHighFreq, CelestialRendererError> {
        let mut high_freq_data_r = toolkit.create_image_full(
            HI_FREQ_RES,
            HI_FREQ_RES,
            HI_FREQ_RES,
            VEImageFormat::R16f,
            &[VEImageUsage::Storage, VEImageUsage::Sampled],
        )?;

        let hi_freq_compute_shader = toolkit.create_shader_module(
            "shaders/compiled/atmosphere/gen-clouds-3d-fbm.comp",
            VEShaderModuleType::Compute,
        )?;

        let buffer = CloudGeneratorHighFreqBuffer::new(toolkit)?;

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
        let view = high_freq_data_r.get_view(VEImageViewCreateInfo::simple_3d())?;
        data_set.bind_image_storage(1, &high_freq_data_r, view)?;

        let compute_stage =
            toolkit.create_compute_stage(&[&data_set_layout], &hi_freq_compute_shader)?;

        compute_stage.begin_recording()?;
        compute_stage.set_descriptor_set(0, &data_set);
        compute_stage.dispatch(
            HI_FREQ_RES / WORKGROUP_SIZE,
            HI_FREQ_RES / WORKGROUP_SIZE,
            HI_FREQ_RES / WORKGROUP_SIZE,
        );
        compute_stage.end_recording()?;

        Ok(CloudGeneratorHighFreq {
            compute_stage,
            data_set_layout,
            data_set,
            buffer,
            high_freq_data_r,
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
