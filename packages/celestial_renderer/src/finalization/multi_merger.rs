use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
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

    pub fn update_inputs(
        &mut self,
        additive: &mut VEImage,
        alpha: &mut VEImage,
        config: &ResolutionConfig,
    ) -> Result<(), RenderingError> {
        let view = additive.get_view(VEImageViewCreateInfo::simple_2d())?;
        self.data_set.bind_image_storage(0, additive, view)?;

        let view = alpha.get_view(VEImageViewCreateInfo::simple_2d())?;
        self.data_set.bind_image_storage(1, alpha, view)?;

        self.compute_stage.begin_recording()?;
        self.compute_stage.set_descriptor_set(0, &self.data_set);
        self.compute_stage.dispatch(
            config.width / WORKGROUP_SIZE,
            config.height / WORKGROUP_SIZE,
            1,
        );
        self.compute_stage.end_recording()?;

        Ok(())
    }
}
