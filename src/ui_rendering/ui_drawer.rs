use crate::celestial_rendering::buffers::common_buffer::CommonBuffer;
use crate::celestial_rendering::errors::RenderingError;
use crate::celestial_rendering::geometry::g_buffer::GBuffer;
use crate::celestial_rendering::scene::mesh::Mesh;
use crate::config::Config;
use crate::ui_rendering::ui_common_buffer::UICommonBuffer;
use crate::ui_rendering::ui_renderer::UIRendererError;
use std::cell::Cell;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
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
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;
use vengine_rs::image::image::{VEImageUsage, VEImageViewCreateInfo};
use vengine_rs::image::image_format::VEImageFormat;

pub struct UIDrawer {
    pub render_stage: VERenderStage,
    pub mesh_set_layout: VEDescriptorSetLayout,
    pub common_set_layout: VEDescriptorSetLayout,
    pub common_set: VEDescriptorSet,
    pub quad_geometry: VEVertexBuffer,
    pub common_buffer: UICommonBuffer,
}

pub const MESH_DRAWER_VERTEX_ATTRIBUTES: [VertexAttribFormat; 2] = [
    VertexAttribFormat::RG32f, // Position
    VertexAttribFormat::RG32f, // UV
];

impl UIDrawer {
    pub fn new(config: &Config, toolkit: &VEToolkit) -> Result<Self, RenderingError> {
        let floats: [f32; 24] = [
            1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 1.0,
        ];
        let bytes: Vec<u8> = floats.iter().flat_map(|x| x.to_le_bytes()).collect();

        let vertex_buffer = toolkit
            .create_vertex_buffer_from_data(bytes, &MESH_DRAWER_VERTEX_ATTRIBUTES)
            .unwrap();

        let quad_geometry = Some(vertex_buffer);

        let mut out_color = toolkit.create_image_full(
            config.width,
            config.height,
            1,
            VEImageFormat::RGBA16f,
            &[VEImageUsage::Storage, VEImageUsage::Sampled],
        )?;

        let out_color_view = out_color.get_view(VEImageViewCreateInfo::simple_2d())?;

        let out_color_attachment = VEAttachment::from_image(
            &out_color,
            out_color_view,
            None,
            Some(clear_color_f32([0.0, 0.0, 0.0, 0.0])),
        )?;

        let item_set_layout = toolkit.create_descriptor_set_layout(&[
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
        })
    }

    pub fn record(&self, meshes: &[&Mesh]) -> Result<(), RenderingError> {
        self.render_stage.begin_recording()?;

        self.render_stage.set_descriptor_set(1, &self.common_set);

        for mesh in meshes {
            self.render_stage
                .set_descriptor_set(0, &mesh.descriptor_set);
            self.render_stage.draw_instanced(&mesh.geometry, 1);
        }

        self.render_stage.end_recording()?;
        Ok(())
    }
}
