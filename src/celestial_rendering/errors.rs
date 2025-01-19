use std::io;
use thiserror::Error;
use vengine_rs::buffer::buffer::VEBufferError;
use vengine_rs::compute::compute_stage::VEComputeStageError;
use vengine_rs::core::descriptor_set::VEDescriptorSetError;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayoutError;
use vengine_rs::core::shader_module::VEShaderModuleError;
use vengine_rs::graphics::attachment::VEAttachmentError;
use vengine_rs::graphics::render_stage::VERenderStageError;
use vengine_rs::graphics::vertex_buffer::VEVertexBufferError;
use vengine_rs::image::image::VEImageError;
use vengine_rs::image::sampler::VESamplerError;

#[derive(Error, Debug)]
pub enum CelestialRendererError {
    #[error("image error")]
    ImageError(#[from] VEImageError),

    #[error("attachment error")]
    AttachmentError(#[from] VEAttachmentError),

    #[error("descriptor set layout error")]
    DescriptorSetLayoutError(#[from] VEDescriptorSetLayoutError),

    #[error("descriptor set error")]
    DescriptorSetError(#[from] VEDescriptorSetError),

    #[error("shader module error")]
    ShaderModuleError(#[from] VEShaderModuleError),

    #[error("sampler error")]
    VESamplerError(#[from] VESamplerError),

    #[error("render stage error")]
    RenderStageError(#[from] VERenderStageError),

    #[error("compute stage error")]
    ComputeStageError(#[from] VEComputeStageError),

    #[error("buffer error")]
    BufferError(#[from] VEBufferError),

    #[error("vertex buffer error")]
    VertexBufferError(#[from] VEVertexBufferError),

    #[error("IO error")]
    IOError(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum PhysicsError {
    #[error("rigid body not found")]
    RigidBodyNotFound,

    #[error("collider not found")]
    ColliderNotFound,
}
