use crate::buffers::common_buffer::CommonBuffer;
use crate::geometry::g_buffer::GBuffer;
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

pub struct WaterIcosphereDrawer {
    pub render_stage: VERenderStage,

    pub data_set_layout: VEDescriptorSetLayout,

    common_set_layout: VEDescriptorSetLayout,
    common_set: VEDescriptorSet,
}

impl Debug for WaterIcosphereDrawer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("WaterIcosphereDrawer")
    }
}

impl WaterIcosphereDrawer {
    pub fn new(
        config: &ResolutionConfig,
        toolkit: &VEToolkit,
        g_buffer: &mut GBuffer,
        common_buffer: &CommonBuffer,
    ) -> Result<WaterIcosphereDrawer, RenderingError> {
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

        let vertex_shader = toolkit.create_shader_module(
            "shaders/compiled/water/water.vert.spv",
            VEShaderModuleType::Vertex,
        )?;

        let fragment_shader = toolkit.create_shader_module(
            "shaders/compiled/water/water.frag.spv",
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
            &[&data_set_layout, &common_set_layout],
            &vertex_shader,
            &fragment_shader,
            &WATER_ICOSPHERE_VERTEX_ATTRIBUTES,
            VEPrimitiveTopology::TriangleList,
            VECullMode::Back,
        )?;

        Ok(WaterIcosphereDrawer {
            render_stage,
            data_set_layout,
            common_set_layout,
            common_set,
        })
    }

    pub fn record(
        &mut self,
        toolkit: &VEToolkit,
        ico: &mut WaterIcosphere,
    ) -> Result<(), RenderingError> {
        self.render_stage.begin_recording()?;

        self.render_stage.set_descriptor_set(0, &ico.data_set);
        self.render_stage.set_descriptor_set(1, &self.common_set);
        ico.icosphere.draw(toolkit, &self.render_stage)?;

        self.render_stage.end_recording()?;
        Ok(())
    }
}
