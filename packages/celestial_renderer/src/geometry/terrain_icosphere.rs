use crate::buffers::icosphere_data_buffer::IcosphereDataBuffer;
use crate::geometry::icosphere_drawer::TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES;
use crate::geometry::icosphere_drawer::WATER_ICOSPHERE_VERTEX_ATTRIBUTES;
use glam::{DMat4, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use planet_generator_library::cubemap_data::CubeMapDataLayer;
use planet_generator_library::generate_icosphere::{
    generate_icosphere_metadata, generate_icosphere_segment, IcosphereMetadataItem, Triangle,
};
use planet_generator_library::interpolated_biome_data::LoadedBiomeData;
use planet_generator_library::load_binary_maps::{
    get_maps_resolution, load_binary_biome_map, load_binary_terrain_map,
};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use renderer_common::errors::RenderingError;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use universe_simulation::simulation::SimulatedBody;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayout;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::render_stage::VERenderStage;
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;
// const LOD_LEVELS: u8 = 3; // 1, 2, 3

static LEVEL_SUBDIVISIONS: [u8; 3] = [4, 5, 6];

struct LoadedGeometry {
    pub terrain_vertex_buffer: Option<VEVertexBuffer>,
    pub water_vertex_buffer: Option<VEVertexBuffer>,
    pub level: u8,
}

pub struct TerrainIcosphere {
    radius: f64,
    loaded_height: CubeMapDataLayer<f64>,
    loaded_biome: CubeMapDataLayer<LoadedBiomeData>,
    base_icosphere: Arc<Vec<Triangle>>,
    currently_loaded: Mutex<HashMap<u16, LoadedGeometry>>,
    metadata: Vec<IcosphereMetadataItem>,
    dir_path: String,
    thresholds: Vec<f64>,
    part_matrices: Vec<DMat4>,

    data_buffer: IcosphereDataBuffer,
    pub data_set: VEDescriptorSet,
}

#[derive(PartialEq, Debug, Eq)]
pub enum DrawMode {
    Terrain,
    Water,
}

static GLOBAL_LOCK: Mutex<()> = Mutex::new(());

struct TerrainData {
    radius: f64,
    dir_path: String,
}

struct WaterData {
    radius: f64,
    water_color: DVec3,
}

impl TerrainIcosphere {
    pub fn new(
        toolkit: &VEToolkit,
        radius: f64,
        dir_path: String,
        thresholds: Vec<f64>,
        base_icosphere: Arc<Vec<Triangle>>,
        data_set_layout: &mut VEDescriptorSetLayout,
    ) -> Result<TerrainIcosphere, RenderingError> {
        let metadata = generate_icosphere_metadata(&base_icosphere, radius);

        let maps_resolutions = get_maps_resolution(&dir_path);

        let part_matrices = vec![DMat4::IDENTITY.clone(); metadata.len()];

        let data_buffer = IcosphereDataBuffer::new(&toolkit)?;
        let data_set = data_set_layout.create_descriptor_set()?;
        data_set.bind_buffer(0, &data_buffer.buffer)?;

        Ok(TerrainIcosphere {
            radius,
            base_icosphere,
            loaded_height: if has_terrain {
                Some(load_binary_terrain_map(radius, &dir_path, maps_resolutions))
            } else {
                None
            },
            loaded_biome: if has_terrain {
                Some(load_binary_biome_map(&dir_path, maps_resolutions))
            } else {
                None
            },
            currently_loaded: Mutex::new(HashMap::new()),
            metadata,
            dir_path,
            thresholds,
            part_matrices,
            data_buffer,
            data_set,
            water_color: water_color.unwrap_or_else(|| DVec3::new(0.0, 0.0, 0.0)),
        })
    }

    pub fn preload(&mut self, toolkit: &VEToolkit) -> Result<(), RenderingError> {
        self.metadata.par_iter().enumerate().for_each(|(i, m)| {
            // println!("ico draw {}", i);
            let m = &self.metadata[i];

            let final_matrix = self.part_matrices[i];

            let distance = final_matrix
                .transform_vector3(DVec3::new(0.0, 0.0, 0.0))
                .length();

            // println!("distance {distance}");

            let mut level = 1;
            if (distance < self.thresholds[1]) {
                level = 2;
            }
            if (distance < self.thresholds[0]) {
                level = 3;
            }

            let exists = self
                .currently_loaded
                .lock()
                .unwrap()
                .contains_key(&m.base_segment);
            if exists {
                let mapped_level = self
                    .currently_loaded
                    .lock()
                    .unwrap()
                    .get(&m.base_segment)
                    .unwrap()
                    .level;
                if (mapped_level != level) {
                    let geometry = self.load_geometry(&toolkit, m.base_segment, level).unwrap();
                    let mut locked = self.currently_loaded.lock().unwrap();
                    let mut mapped_mut = locked.get_mut(&m.base_segment).unwrap();
                    mapped_mut.level = level;
                    mapped_mut.terrain_vertex_buffer = geometry.terrain_vertex_buffer;
                    mapped_mut.water_vertex_buffer = geometry.water_vertex_buffer;
                }
            } else {
                let geometry = self.load_geometry(&toolkit, m.base_segment, level).unwrap();
                self.currently_loaded
                    .lock()
                    .unwrap()
                    .insert(m.base_segment, geometry);
            }
        });

        Ok(())
    }

    pub fn update_buffer(
        &mut self,
        camera_position: &DecimalVector3d,
        simulated_body: &SimulatedBody,
    ) -> Result<(), RenderingError> {
        let relative_camera_position = &simulated_body.position - camera_position;

        let rotation_matrix = simulated_body.orientation.as_dmat4().inverse();
        let world_translation_matrix = DMat4::from_translation(relative_camera_position.to_dvec3());
        let pre_final_matrix = world_translation_matrix * rotation_matrix;

        for i in 0..self.metadata.len() {
            let metadata = &self.metadata[i];
            let model_offset_matrix = DMat4::from_translation(metadata.center);
            self.part_matrices[i] = pre_final_matrix * model_offset_matrix;
        }

        let body_center_camera_space = &simulated_body.position - camera_position;

        self.data_buffer.update(
            self.water_color,
            body_center_camera_space.to_dvec3(),
            &self.part_matrices,
        )?;

        Ok(())
    }

    pub fn draw(&mut self, stage: &VERenderStage, mode: DrawMode) -> Result<(), RenderingError> {
        if mode == DrawMode::Terrain && !self.has_terrain {
            return Ok(());
        }
        if mode == DrawMode::Water && !self.has_water {
            return Ok(());
        }
        self.metadata.par_iter().enumerate().for_each(|(i, m)| {
            let m = &self.metadata[i];

            let locked = self.currently_loaded.lock().unwrap();
            if let Some(mapped) = locked.get(&m.base_segment) {
                let vb = match mode {
                    DrawMode::Terrain => &mapped.terrain_vertex_buffer,
                    DrawMode::Water => &mapped.water_vertex_buffer,
                };
                if vb.is_some() {
                    stage.draw_instanced(vb.as_ref().unwrap(), 1);
                }
            }
        });

        Ok(())
    }

    fn load_geometry(
        &self,
        toolkit: &VEToolkit,
        base_segment: u16,
        level: u8,
    ) -> Result<LoadedGeometry, RenderingError> {
        let subdivisions = LEVEL_SUBDIVISIONS[level as usize - 1];

        let segment = generate_icosphere_segment(
            self.has_terrain,
            self.has_water,
            &self.base_icosphere,
            self.loaded_height.as_ref(),
            self.loaded_biome.as_ref(),
            base_segment,
            self.radius,
            subdivisions,
        );

        //let _exclusivity_lock = GLOBAL_LOCK.lock().unwrap(); // this probably could be done better if i did it in vengine

        let terrain_buffer = if self.has_terrain {
            Some(toolkit.create_vertex_buffer_from_data(
                segment.terrain_vertex_buffer.unwrap(),
                &TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES,
            )?)
        } else {
            None
        };

        let water_buffer = if self.has_water {
            Some(toolkit.create_vertex_buffer_from_data(
                segment.water_vertex_buffer.unwrap(),
                &WATER_ICOSPHERE_VERTEX_ATTRIBUTES,
            )?)
        } else {
            None
        };

        //drop(_exclusivity_lock);

        Ok(LoadedGeometry {
            terrain_vertex_buffer: terrain_buffer,
            water_vertex_buffer: water_buffer,
            level,
        })
    }
}
