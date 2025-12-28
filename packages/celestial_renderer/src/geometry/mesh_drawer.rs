use crate::buffers::common_buffer::CommonBuffer;
use crate::geometry::g_buffer::GBuffer;
use crate::scene::mesh::Mesh;
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::helpers::clear_depth;
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::attachment::VEAttachment;
use vengine_rs::graphics::render_stage::{VECullMode, VEPrimitiveTopology, VERenderStage};
use vengine_rs::graphics::vertex_attributes::VertexAttribFormat;
use vengine_rs::image::image::VEImageViewCreateInfo;

pub struct MeshDrawer {
    config: ResolutionConfig,

    pub render_stage: VERenderStage,
    pub mesh_set_layout: VEDescriptorSetLayout,
    pub common_set_layout: VEDescriptorSetLayout,
    pub common_set: VEDescriptorSet,

    color_rgb_roughness_a_attachment: VEAttachment,
    normal_rgb_distance_a_attachment: VEAttachment,
    emission_rgb_metalness_a_attachment: VEAttachment,
    shared_depth_buffer_attachment: VEAttachment,
}

pub const MESH_DRAWER_VERTEX_ATTRIBUTES: [VertexAttribFormat; 4] = [
    VertexAttribFormat::RGB32f,
    VertexAttribFormat::RGB32f,
    VertexAttribFormat::RG32f,
    VertexAttribFormat::RGBA32f,
];

impl MeshDrawer {
    pub fn new(
        config: &ResolutionConfig,
        toolkit: &VEToolkit,
        g_buffer: &mut GBuffer,
        common_buffer: &CommonBuffer,
    ) -> Result<MeshDrawer, RenderingError> {
        let color_rgb_roughness_a_view = g_buffer
            .color_rgb_roughness_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;

        let color_rgb_roughness_a_attachment = VEAttachment::from_image(
            &g_buffer.color_rgb_roughness_a,
            color_rgb_roughness_a_view,
            None,
            None,
        )?;

        let emission_rgb_metalness_a_view = g_buffer
            .emission_rgb_metalness_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;

        let emission_rgb_metalness_a_attachment = VEAttachment::from_image(
            &g_buffer.emission_rgb_metalness_a,
            emission_rgb_metalness_a_view,
            None,
            None,
        )?;

        let normal_rgb_distance_a_view = g_buffer
            .normal_rgb_distance_a
            .get_view(VEImageViewCreateInfo::simple_2d())?;

        let normal_rgb_distance_a_attachment = VEAttachment::from_image(
            &g_buffer.normal_rgb_distance_a,
            normal_rgb_distance_a_view,
            None,
            None,
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

        let mut common_set_layout =
            toolkit.create_descriptor_set_layout(&[VEDescriptorSetLayoutField {
                // data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::AllGraphics,
            }])?;

        let common_set = common_set_layout.create_descriptor_set()?;
        common_set.bind_buffer(0, &common_buffer.buffer)?;

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
                &shared_depth_buffer_attachment,
            ],
            &[&mesh_set_layout, &common_set_layout],
            &vertex_shader,
            &fragment_shader,
            &MESH_DRAWER_VERTEX_ATTRIBUTES,
            VEPrimitiveTopology::TriangleList,
            VECullMode::Back,
        )?;

        Ok(MeshDrawer {
            render_stage,
            mesh_set_layout,
            common_set_layout,
            common_set,
            config: config.clone(),
            color_rgb_roughness_a_attachment,
            normal_rgb_distance_a_attachment,
            emission_rgb_metalness_a_attachment,
            shared_depth_buffer_attachment,
        })
    }

    pub fn recreate_stage(&mut self, toolkit: &VEToolkit) -> Result<(), RenderingError> {
        let vertex_shader = toolkit.create_shader_module(
            "shaders/compiled/mesh/mesh.vert.spv",
            VEShaderModuleType::Vertex,
        )?;

        let fragment_shader = toolkit.create_shader_module(
            "shaders/compiled/mesh/mesh.frag.spv",
            VEShaderModuleType::Fragment,
        )?;

        self.render_stage = toolkit.create_render_stage(
            self.config.width,
            self.config.height,
            &[
                &self.color_rgb_roughness_a_attachment,
                &self.normal_rgb_distance_a_attachment,
                &self.emission_rgb_metalness_a_attachment,
                &self.shared_depth_buffer_attachment,
            ],
            &[&self.mesh_set_layout, &self.common_set_layout],
            &vertex_shader,
            &fragment_shader,
            &MESH_DRAWER_VERTEX_ATTRIBUTES,
            VEPrimitiveTopology::TriangleList,
            VECullMode::Back,
        )?;

        Ok(())
    }

    pub fn record(
        &self,
        stage: &VERenderStage,
        command_buffer: &VECommandBuffer,
        meshes: &[&Mesh],
    ) {
        stage.bind(command_buffer);

        self.render_stage
            .set_descriptor_set(command_buffer, 1, &self.common_set);

        for mesh in meshes {
            self.render_stage
                .set_descriptor_set(command_buffer, 0, &mesh.descriptor_set);
            mesh.geometry.draw_instanced(command_buffer, 1);
        }

        stage.end_render_pass(command_buffer);
    }
}
