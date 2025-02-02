use crate::buffers::terrain_icosphere_data_buffer::TerrainIcosphereDataBuffer;
use crate::geometry::icosphere::Icosphere;
use crate::geometry::terrain_icosphere_drawer::TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES;
use glam::DQuat;
use math::decimal_vector_3d::DecimalVector3d;
use renderer_common::errors::RenderingError;
use universe_simulation::simulation::SimulatedBody;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayout;
use vengine_rs::core::toolkit::VEToolkit;

pub struct TerrainIcosphere {
    pub icosphere: Icosphere,
    pub data_buffer: TerrainIcosphereDataBuffer,
    pub data_set: VEDescriptorSet,
}

impl TerrainIcosphere {
    pub fn new(
        toolkit: &VEToolkit,
        data_set_layout: &mut VEDescriptorSetLayout,
        dir_path: String,
        thresholds: Vec<f64>,
    ) -> Result<Self, RenderingError> {
        let icosphere = Icosphere::new(
            dir_path,
            thresholds,
            TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES.to_vec(),
        )?;
        let data_buffer = TerrainIcosphereDataBuffer::new(&toolkit)?;
        let data_set = data_set_layout.create_descriptor_set()?;
        data_set.bind_buffer(0, &data_buffer.buffer)?;
        Ok(Self {
            icosphere,
            data_buffer,
            data_set,
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

        self.data_buffer.update(matrices)?;

        Ok(())
    }
}
