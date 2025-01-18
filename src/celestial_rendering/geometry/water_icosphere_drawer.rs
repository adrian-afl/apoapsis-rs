use crate::celestial_rendering::buffers::celestial_body_buffer::CelestialBodyBuffer;
use crate::celestial_rendering::buffers::common_buffer::CommonBuffer;
use crate::celestial_rendering::buffers::water_icosphere_data_buffer::WaterIcosphereDataBuffer;
use crate::celestial_rendering::errors::CelestialRendererError;
use crate::celestial_rendering::geometry::g_buffer::GBuffer;
use crate::celestial_rendering::geometry::icosphere::Icosphere;
use crate::celestial_rendering::scene::camera::Camera;
use crate::config::Config;
use crate::math::decimal_vector_3d::DecimalVector3d;
use crate::simulation::simulation::SimulatedBody;
use glam::{DQuat, DVec3};
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
    icosphere: Icosphere,
    pub render_stage: VERenderStage,

    buffer: WaterIcosphereDataBuffer,

    data_set_layout: VEDescriptorSetLayout,
    data_set: VEDescriptorSet,

    common_set_layout: VEDescriptorSetLayout,
    common_set: VEDescriptorSet,

    water_color: DVec3,
}

impl Debug for WaterIcosphereDrawer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("WaterIcosphereDrawer")
    }
}

impl WaterIcosphereDrawer {
    pub fn new(
        config: &Config,
        toolkit: &VEToolkit,
        g_buffer: &mut GBuffer,
        common_buffer: &CommonBuffer,
        dir_path: String,
        thresholds: Vec<f64>,
        water_color: DVec3,
    ) -> Result<WaterIcosphereDrawer, CelestialRendererError> {
        let vertex_attributes = vec![
            VertexAttribFormat::RGB32f,
            VertexAttribFormat::R16u,
            VertexAttribFormat::Padding16,
        ];

        let icosphere = Icosphere::new(dir_path, thresholds, vertex_attributes.clone())?;

        // no clear!! mesh stage clears!

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

        let data_buffer = WaterIcosphereDataBuffer::new(&toolkit)?;

        let mut data_set_layout =
            toolkit.create_descriptor_set_layout(&[VEDescriptorSetLayoutField {
                // data buffer
                binding: 0,
                typ: VEDescriptorSetFieldType::StorageBuffer,
                stage: VEDescriptorSetFieldStage::AllGraphics,
            }])?;

        let data_set = data_set_layout.create_descriptor_set()?;
        data_set.bind_buffer(0, &data_buffer.buffer)?;

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
            &vertex_attributes,
            VEPrimitiveTopology::TriangleList,
            VECullMode::Back,
        )?;

        Ok(WaterIcosphereDrawer {
            icosphere,
            buffer: data_buffer,
            render_stage,
            data_set_layout,
            data_set,
            common_set_layout,
            common_set,
            water_color,
        })
    }

    pub fn update_buffer(
        &mut self,
        camera_position: &DecimalVector3d,
        simulated_body: &SimulatedBody,
    ) -> Result<(), CelestialRendererError> {
        let matrices = self.icosphere.update_and_get_part_matrices(
            &camera_position,
            &simulated_body.position,
            DQuat::from_mat4(&simulated_body.orientation.as_dmat4()),
        );

        let body_center_camera_space = &simulated_body.position - camera_position;

        self.buffer.update(
            self.water_color,
            body_center_camera_space.to_dvec3(),
            matrices,
        )?;

        Ok(())
    }

    pub fn record(&mut self, toolkit: &VEToolkit) -> Result<(), CelestialRendererError> {
        self.render_stage.begin_recording()?;

        self.render_stage.set_descriptor_set(0, &self.data_set);
        self.render_stage.set_descriptor_set(1, &self.common_set);
        self.icosphere.draw(toolkit, &self.render_stage)?;

        self.render_stage.end_recording()?;
        Ok(())
    }
}
