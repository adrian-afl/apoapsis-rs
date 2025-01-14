use crate::celestial_rendering::errors::CelestialRendererError;
use crate::celestial_rendering::geometry::g_buffer::GBuffer;
use crate::config::Config;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::helpers::{clear_color_f32, clear_depth};
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::attachment::VEAttachment;
use vengine_rs::graphics::render_stage::{VECullMode, VEPrimitiveTopology, VERenderStage};
use vengine_rs::graphics::vertex_attributes::VertexAttribFormat;
use vengine_rs::image::image::VEImageViewCreateInfo;

pub struct MeshDrawer {
    pub render_stage: VERenderStage,
    pub mesh_set_layout: VEDescriptorSetLayout,
}

impl MeshDrawer {
    pub fn new(
        config: &Config,
        toolkit: &VEToolkit,
        g_buffer: &mut GBuffer,
    ) -> Result<MeshDrawer, CelestialRendererError> {
        let vertex_attributes = [
            VertexAttribFormat::RGB32f,
            VertexAttribFormat::RGB32f,
            VertexAttribFormat::RG32f,
            VertexAttribFormat::RGBA32f,
        ];

        // Mesh stage is first and clears the GBuffer, so attachments here should clear

        let color_rgb_roughness_a_view = g_buffer
            .color_rgb_roughness_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;

        let color_rgb_roughness_a_attachment = VEAttachment::from_image(
            &g_buffer.color_rgb_roughness_a,
            color_rgb_roughness_a_view,
            None,
            Some(clear_color_f32([0.0, 0.0, 0.0, 0.0])),
        )?;

        let emission_rgb_metalness_a_view = g_buffer
            .emission_rgb_metalness_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;

        let emission_rgb_metalness_a_attachment = VEAttachment::from_image(
            &g_buffer.emission_rgb_metalness_a,
            emission_rgb_metalness_a_view,
            None,
            Some(clear_color_f32([0.0, 0.0, 0.0, 0.0])),
        )?;

        let normal_rgb_distance_a_view = g_buffer
            .normal_rgb_distance_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;

        let normal_rgb_distance_a_attachment = VEAttachment::from_image(
            &g_buffer.normal_rgb_distance_a,
            normal_rgb_distance_a_view,
            None,
            Some(clear_color_f32([0.0, 0.0, 0.0, 0.0])),
        )?;

        let shared_depth_buffer_view = g_buffer
            .shared_depth_buffer
            .get_view(VEImageViewCreateInfo::simple_2d())?;

        let shared_depth_buffer_attachment = VEAttachment::from_image(
            &g_buffer.shared_depth_buffer,
            shared_depth_buffer_view,
            None,
            Some(clear_depth(1.0)),
        )?;

        let mesh_set_layout = toolkit.create_descriptor_set_layout(&[
            VEDescriptorSetLayoutField {
                // data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::AllGraphics,
            },
            VEDescriptorSetLayoutField {
                // color tex
                binding: 1,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
            VEDescriptorSetLayoutField {
                // roughness tex
                binding: 2,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
            VEDescriptorSetLayoutField {
                // metalness tex
                binding: 3,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
            VEDescriptorSetLayoutField {
                // emission tex
                binding: 4,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
            VEDescriptorSetLayoutField {
                // normal tex
                binding: 5,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
            VEDescriptorSetLayoutField {
                // bump tex
                binding: 6,
                typ: VEDescriptorSetFieldType::Sampler,
                stage: VEDescriptorSetFieldStage::Fragment,
            },
        ])?;

        let vertex_shader = toolkit.create_shader_module(
            "shaders/compiled/mesh/mesh.vert.spv",
            VEShaderModuleType::Vertex,
        )?;

        let fragment_shader = toolkit.create_shader_module(
            "shaders/compiled/mesh/mesh.frag.spv",
            VEShaderModuleType::Fragment,
        )?;

        let render_stage = toolkit.create_render_stage(
            config.width,
            config.height,
            &[
                &color_rgb_roughness_a_attachment,
                &normal_rgb_distance_a_attachment,
                &emission_rgb_metalness_a_attachment,
            ],
            &[&mesh_set_layout],
            &vertex_shader,
            &fragment_shader,
            &vertex_attributes,
            VEPrimitiveTopology::TriangleList,
            VECullMode::Back,
        )?;

        Ok(MeshDrawer {
            render_stage,
            mesh_set_layout,
        })
    }
}
