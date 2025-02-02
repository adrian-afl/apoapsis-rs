use crate::buffers::water_icosphere_data_buffer::WaterIcosphereDataBuffer;
use crate::geometry::icosphere::Icosphere;
use crate::geometry::terrain_icosphere_drawer::TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES;
use crate::geometry::water_icosphere_drawer::WATER_ICOSPHERE_VERTEX_ATTRIBUTES;
use glam::{DQuat, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use renderer_common::errors::RenderingError;
use universe_simulation::simulation::SimulatedBody;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayout;
use vengine_rs::core::toolkit::VEToolkit;

pub struct WaterIcosphere {
    pub icosphere: Icosphere,
    pub data_buffer: WaterIcosphereDataBuffer,
    pub data_set: VEDescriptorSet,
    pub water_color: DVec3,
}

impl WaterIcosphere {
    pub fn new(
        toolkit: &VEToolkit,
        data_set_layout: &mut VEDescriptorSetLayout,
        dir_path: String,
        thresholds: Vec<f64>,
        water_color: DVec3,
    ) -> Result<Self, RenderingError> {
        let icosphere = Icosphere::new(
            dir_path,
            thresholds,
            WATER_ICOSPHERE_VERTEX_ATTRIBUTES.to_vec(),
        )?;
        let data_buffer = WaterIcosphereDataBuffer::new(&toolkit)?;
        let data_set = data_set_layout.create_descriptor_set()?;
        data_set.bind_buffer(0, &data_buffer.buffer)?;
        Ok(Self {
            icosphere,
            data_buffer,
            data_set,
            water_color,
        })
    }

    pub fn update_buffer(
        &mut self,
        camera_position: &DecimalVector3d,
        simulated_body: &SimulatedBody,
    ) -> Result<(), RenderingError> {
        let matrices = self.icosphere.update_and_get_part_matrices(
            &camera_position,
            &simulated_body.position,
            DQuat::from_mat4(&simulated_body.orientation.as_dmat4()),
        );

        let body_center_camera_space = &simulated_body.position - camera_position;

        self.data_buffer.update(
            self.water_color,
            body_center_camera_space.to_dvec3(),
            matrices,
        )?;

        Ok(())
    }
}
