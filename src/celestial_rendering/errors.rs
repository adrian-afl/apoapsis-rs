use thiserror::Error;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayoutError;
use vengine_rs::core::shader_module::VEShaderModuleError;
use vengine_rs::graphics::attachment::VEAttachmentError;
use vengine_rs::graphics::render_stage::VERenderStageError;
use vengine_rs::image::image::VEImageError;

#[derive(Error, Debug)]
pub enum CelestialRendererError {
    #[error("image error")]
    ImageError(#[from] VEImageError),

    #[error("attachment error")]
    AttachmentError(#[from] VEAttachmentError),

    #[error("descriptor set layout error")]
    DescriptorSetLayoutError(#[from] VEDescriptorSetLayoutError),

    #[error("shader module error")]
    ShaderModuleError(#[from] VEShaderModuleError),

    #[error("render stage error")]
    RenderStageError(#[from] VERenderStageError),
}
