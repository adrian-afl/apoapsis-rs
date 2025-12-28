use crate::buffers::terrain_icosphere_data_buffer::TerrainIcosphereDataBuffer;
use crate::geometry::common_icosphere::{
    update_icosphere_matrices, which_part_to_preload, IcosphereLoadedGeometry,
    PreloadDetectionResultAction, PreloadResult, ICO_LEVEL_SUBDIVISIONS,
};
use crate::geometry::icosphere_drawer::TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES;
use glam::{DMat4, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use planet_generator_library::cubemap_data::CubeMapDataLayer;
use planet_generator_library::generate_icosphere::{
    generate_icosphere_metadata, IcosphereMetadataItem, IcosphereSegmentGenerator, Triangle,
};
use planet_generator_library::interpolated_biome_data::LoadedBiomeData;
use planet_generator_library::load_binary_maps::{
    get_terrain_maps_resolution, load_binary_biome_map, load_binary_terrain_map,
};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use renderer_common::errors::RenderingError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use universe_simulation::simulation::SimulatedBody;
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayout;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::render_stage::VERenderStage;

struct LoadedTerrainData {
    radius: f64,
    loaded_height: CubeMapDataLayer<f64>,
    loaded_biome: CubeMapDataLayer<LoadedBiomeData>,
    metadata: Vec<IcosphereMetadataItem>,
    part_matrices: Vec<DMat4>,
}

pub struct TerrainIcosphere {
    generator: Arc<IcosphereSegmentGenerator>,
    base_icosphere: Arc<Vec<Triangle>>,
    currently_loaded: Mutex<HashMap<u16, IcosphereLoadedGeometry>>,

    loaded_data: LoadedTerrainData,

    data_buffer: TerrainIcosphereDataBuffer,
    pub data_set: VEDescriptorSet,
}

pub struct TerrainData {
    pub radius: f64,
    pub dir_path: String,
}

impl TerrainIcosphere {
    pub fn new(
        toolkit: &VEToolkit,
        generator: Arc<IcosphereSegmentGenerator>,
        terrain_data: TerrainData,
        base_icosphere: Arc<Vec<Triangle>>,
        data_set_layout: &mut VEDescriptorSetLayout,
    ) -> Result<TerrainIcosphere, RenderingError> {
        let metadata = generate_icosphere_metadata(&base_icosphere, terrain_data.radius);
        let part_matrices = vec![DMat4::IDENTITY; metadata.len()];
        let maps_resolutions = get_terrain_maps_resolution(&terrain_data.dir_path);
        let loaded_data = LoadedTerrainData {
            radius: terrain_data.radius,
            part_matrices,
            metadata,
            loaded_height: load_binary_terrain_map(
                terrain_data.radius,
                &terrain_data.dir_path,
                maps_resolutions,
            ),
            loaded_biome: load_binary_biome_map(&terrain_data.dir_path, maps_resolutions),
        };

        let data_buffer = TerrainIcosphereDataBuffer::new(toolkit)?;
        let data_set = data_set_layout.create_descriptor_set()?;
        data_set.bind_buffer(0, &data_buffer.buffer)?;

        Ok(TerrainIcosphere {
            generator,
            loaded_data,
            base_icosphere,

            currently_loaded: Mutex::new(HashMap::new()),
            data_buffer,
            data_set,
        })
    }

    pub fn get_radius_at_normal(&self, normal: DVec3) -> f64 {
        self.loaded_data.loaded_height.get(normal)
    }

    pub fn preload(&mut self, toolkit: &VEToolkit) -> Result<PreloadResult, RenderingError> {
        let which_to_preload = which_part_to_preload(
            &self.loaded_data.metadata,
            &self.loaded_data.part_matrices,
            &self.currently_loaded.lock().unwrap(),
        );

        which_to_preload.par_iter().for_each(|x| {
            let geometry = self
                .load_geometry(toolkit, x.base_segment, x.level)
                .unwrap();

            match x.action {
                PreloadDetectionResultAction::Insert => {
                    self.currently_loaded
                        .lock()
                        .unwrap()
                        .insert(x.base_segment, geometry);
                }
                PreloadDetectionResultAction::Update => {
                    let mut locked = self.currently_loaded.lock().unwrap();
                    let mapped_mut = locked.get_mut(&x.base_segment).unwrap();
                    mapped_mut.level = x.level;
                    mapped_mut.vertex_buffer = geometry.vertex_buffer;
                }
            }
        });

        Ok(if !which_to_preload.is_empty() {
            PreloadResult::ChangesMade
        } else {
            PreloadResult::NotChanged
        })
    }

    pub fn preload_lowest_quality(
        &mut self,
        toolkit: &VEToolkit,
    ) -> Result<PreloadResult, RenderingError> {
        let mut locked = self.currently_loaded.lock().unwrap();
        let mut anything_changed = false;
        for keyval in locked.iter_mut() {
            if keyval.1.level != 1 {
                let geometry = self.load_geometry(toolkit, *keyval.0, 1)?;
                keyval.1.level = 1;
                keyval.1.vertex_buffer = geometry.vertex_buffer;
                anything_changed = true;
            }
        }

        Ok(if anything_changed {
            PreloadResult::ChangesMade
        } else {
            PreloadResult::NotChanged
        })
    }

    pub fn update_buffer(
        &mut self,
        camera_position: &DecimalVector3d,
        simulated_body: &SimulatedBody,
    ) -> Result<(), RenderingError> {
        update_icosphere_matrices(
            camera_position,
            simulated_body,
            &self.loaded_data.metadata,
            &mut self.loaded_data.part_matrices,
        );

        self.data_buffer.update(&self.loaded_data.part_matrices)?;

        Ok(())
    }

    pub fn record(
        &self,
        stage: &VERenderStage,
        command_buffer: &VECommandBuffer,
        common_set: &VEDescriptorSet,
    ) {
        self.data_buffer.record_copy_from_staging(command_buffer);

        stage.bind(command_buffer);
        stage.set_descriptor_set(command_buffer, 0, &self.data_set);
        stage.set_descriptor_set(command_buffer, 1, common_set);

        let locked = self.currently_loaded.lock().unwrap();
        self.loaded_data.metadata.iter().for_each(|m| {
            if let Some(mapped) = locked.get(&m.base_segment) {
                mapped.vertex_buffer.draw_instanced(command_buffer, 1);
            }
        });

        stage.end_render_pass(command_buffer);
    }

    fn load_geometry(
        &self,
        toolkit: &VEToolkit,
        base_segment: u16,
        level: u8,
    ) -> Result<IcosphereLoadedGeometry, RenderingError> {
        let subdivisions = ICO_LEVEL_SUBDIVISIONS[level as usize - 1];

        let segment = self.generator.generate_terrain(
            base_segment,
            self.loaded_data.radius,
            subdivisions,
            &self.loaded_data.loaded_height,
            &self.loaded_data.loaded_biome,
        );

        let vertex_buffer = toolkit
            .create_vertex_buffer_from_data(segment, &TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES)?;

        Ok(IcosphereLoadedGeometry {
            vertex_buffer,
            level,
        })
    }
}
