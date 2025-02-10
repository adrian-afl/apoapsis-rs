use crate::buffers::common_buffer::CommonBuffer;
use crate::geometry::g_buffer::GBuffer;
use crate::geometry::terrain_icosphere::TerrainIcosphere;
use crate::geometry::water_icosphere::WaterIcosphere;
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use std::fmt::{Debug, Formatter};
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::{
    VEDescriptorSetFieldStage, VEDescriptorSetFieldType, VEDescriptorSetLayout,
    VEDescriptorSetLayoutField,
};
use vengine_rs::core::shader_module::VEShaderModuleType;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::attachment::VEAttachment;
use vengine_rs::graphics::render_stage::{VECullMode, VEPrimitiveTopology, VERenderStage};
use vengine_rs::graphics::vertex_attributes::VertexAttribFormat;
use vengine_rs::image::image::VEImageViewCreateInfo;

pub static TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES: [VertexAttribFormat; 6] = [
    VertexAttribFormat::RGB32f,    // pos
    VertexAttribFormat::RGB8inorm, // normal
    VertexAttribFormat::Padding8,
    VertexAttribFormat::RGBA8unorm, // color roughness
    VertexAttribFormat::R16u,       // part number
    VertexAttribFormat::Padding16,
];

pub static WATER_ICOSPHERE_VERTEX_ATTRIBUTES: [VertexAttribFormat; 3] = [
    VertexAttribFormat::RGB32f,
    VertexAttribFormat::R16u,
    VertexAttribFormat::Padding16,
];

pub struct IcosphereDrawer {
    pub terrain_render_stage: VERenderStage,
    pub water_render_stage: VERenderStage,

    pub data_set_layout: VEDescriptorSetLayout,

    common_set_layout: VEDescriptorSetLayout,
    common_set: VEDescriptorSet,
}

impl Debug for IcosphereDrawer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("IcosphereDrawer")
    }
}

impl IcosphereDrawer {
    pub fn new(
        toolkit: &VEToolkit,
        config: &ResolutionConfig,
        g_buffer: &mut GBuffer,
        common_buffer: &CommonBuffer,
    ) -> Result<Self, RenderingError> {
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
            None,
        )?;

        let mut data_set_layout =
            toolkit.create_descriptor_set_layout(&[VEDescriptorSetLayoutField {
                // data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::StorageBuffer,
                stage: VEDescriptorSetFieldStage::AllGraphics,
            }])?;

        let mut common_set_layout =
            toolkit.create_descriptor_set_layout(&[VEDescriptorSetLayoutField {
                // data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::UniformBuffer,
                stage: VEDescriptorSetFieldStage::AllGraphics,
            }])?;

        let common_set = common_set_layout.create_descriptor_set()?;
        common_set.bind_buffer(0, &common_buffer.buffer)?;

        let terrain_vertex_shader = toolkit.create_shader_module(
            "shaders/compiled/terrain/terrain.vert.spv",
            VEShaderModuleType::Vertex,
        )?;

        let terrain_fragment_shader = toolkit.create_shader_module(
            "shaders/compiled/terrain/terrain.frag.spv",
            VEShaderModuleType::Fragment,
        )?;

        let terrain_render_stage = toolkit.create_render_stage(
            config.width,
            config.height,
            &[
                &color_rgb_roughness_a_attachment,
                &normal_rgb_distance_a_attachment,
                &emission_rgb_metalness_a_attachment,
                &shared_depth_buffer_attachment,
            ],
            &[&data_set_layout, &common_set_layout],
            &terrain_vertex_shader,
            &terrain_fragment_shader,
            &TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES,
            VEPrimitiveTopology::TriangleList,
            VECullMode::Back,
        )?;

        let water_vertex_shader = toolkit.create_shader_module(
            "shaders/compiled/water/water.vert.spv",
            VEShaderModuleType::Vertex,
        )?;

        let water_fragment_shader = toolkit.create_shader_module(
            "shaders/compiled/water/water.frag.spv",
            VEShaderModuleType::Fragment,
        )?;

        let water_render_stage = toolkit.create_render_stage(
            config.width,
            config.height,
            &[
                &color_rgb_roughness_a_attachment,
                &normal_rgb_distance_a_attachment,
                &emission_rgb_metalness_a_attachment,
                &shared_depth_buffer_attachment,
            ],
            &[&data_set_layout, &common_set_layout],
            &water_vertex_shader,
            &water_fragment_shader,
            &WATER_ICOSPHERE_VERTEX_ATTRIBUTES,
            VEPrimitiveTopology::TriangleList,
            VECullMode::Back,
        )?;

        Ok(Self {
            data_set_layout,
            common_set_layout,
            terrain_render_stage,
            water_render_stage,
            common_set,
        })
    }

    pub fn record_terrain(&mut self, ico: &mut TerrainIcosphere) -> Result<(), RenderingError> {
        self.terrain_render_stage.begin_recording()?;

        self.terrain_render_stage
            .set_descriptor_set(0, &ico.data_set);
        self.terrain_render_stage
            .set_descriptor_set(1, &self.common_set);
        ico.draw(&self.terrain_render_stage)?;

        self.terrain_render_stage.end_recording()?;

        Ok(())
    }

    pub fn record_water(&mut self, ico: &mut WaterIcosphere) -> Result<(), RenderingError> {
        self.water_render_stage.begin_recording()?;

        self.water_render_stage.set_descriptor_set(0, &ico.data_set);
        self.water_render_stage
            .set_descriptor_set(1, &self.common_set);
        ico.draw(&self.water_render_stage)?;

        self.water_render_stage.end_recording()?;

        Ok(())
    }
}
