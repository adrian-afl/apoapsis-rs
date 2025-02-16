use crate::buffers::water_icosphere_data_buffer::WaterIcosphereDataBuffer;
use crate::geometry::common_icosphere::{
    update_icosphere_matrices, which_part_to_preload, IcosphereLoadedGeometry,
    PreloadDetectionResultAction, ICO_LEVEL_SUBDIVISIONS,
};
use crate::geometry::icosphere_drawer::WATER_ICOSPHERE_VERTEX_ATTRIBUTES;
use glam::{DMat4, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use planet_generator_library::cubemap_data::CubeMapDataLayer;
use planet_generator_library::generate_icosphere::{
    generate_icosphere_metadata, IcosphereMetadataItem, IcosphereSegmentGenerator, Triangle,
};
use planet_generator_library::load_binary_maps::{
    get_water_maps_resolution, load_binary_water_map,
};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use renderer_common::errors::RenderingError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use universe_simulation::simulation::SimulatedBody;
use vengine_rs::core::descriptor_set::VEDescriptorSet;
use vengine_rs::core::descriptor_set_layout::VEDescriptorSetLayout;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::render_stage::VERenderStage;

struct LoadedWaterData {
    radius: f64,
    loaded_height: CubeMapDataLayer<f64>,
    water_color: DVec3,
    metadata: Vec<IcosphereMetadataItem>,
    part_matrices: Vec<DMat4>,
}

pub struct WaterIcosphere {
    generator: Arc<IcosphereSegmentGenerator>,
    base_icosphere: Arc<Vec<Triangle>>,
    currently_loaded: Mutex<HashMap<u16, IcosphereLoadedGeometry>>,

    loaded_data: LoadedWaterData,

    data_buffer: WaterIcosphereDataBuffer,
    pub data_set: VEDescriptorSet,
}

pub struct WaterData {
    pub radius: f64,
    pub water_color: DVec3,
    pub dir_path: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum PreloadResult {
    ChangesMade,
    NotChanged,
}

impl WaterIcosphere {
    pub fn new(
        toolkit: &VEToolkit,
        generator: Arc<IcosphereSegmentGenerator>,
        water_data: WaterData,
        base_icosphere: Arc<Vec<Triangle>>,
        data_set_layout: &mut VEDescriptorSetLayout,
    ) -> Result<WaterIcosphere, RenderingError> {
        let metadata = generate_icosphere_metadata(&base_icosphere, water_data.radius);
        let part_matrices = vec![DMat4::IDENTITY.clone(); metadata.len()];
        let maps_resolutions = get_water_maps_resolution(&water_data.dir_path);
        let loaded_data = LoadedWaterData {
            radius: water_data.radius,
            part_matrices,
            metadata,
            water_color: water_data.water_color,
            loaded_height: load_binary_water_map(
                water_data.radius,
                &water_data.dir_path,
                maps_resolutions,
            ),
        };

        let data_buffer = WaterIcosphereDataBuffer::new(&toolkit)?;
        let data_set = data_set_layout.create_descriptor_set()?;
        data_set.bind_buffer(0, &data_buffer.buffer)?;

        Ok(WaterIcosphere {
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
                .load_geometry(&toolkit, x.base_segment, x.level)
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

        Ok(if which_to_preload.len() > 0 {
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

        let body_center_camera_space = &simulated_body.position - camera_position;

        self.data_buffer.update(
            self.loaded_data.water_color,
            body_center_camera_space.to_dvec3(),
            &self.loaded_data.part_matrices,
        )?;

        Ok(())
    }

    pub fn draw(&mut self, stage: &VERenderStage) -> Result<(), RenderingError> {
        self.loaded_data.metadata.par_iter().for_each(|m| {
            let locked = self.currently_loaded.lock().unwrap();
            if let Some(mapped) = locked.get(&m.base_segment) {
                stage.draw_instanced(&mapped.vertex_buffer, 1);
            }
        });

        Ok(())
    }

    fn load_geometry(
        &self,
        toolkit: &VEToolkit,
        base_segment: u16,
        level: u8,
    ) -> Result<IcosphereLoadedGeometry, RenderingError> {
        let subdivisions = ICO_LEVEL_SUBDIVISIONS[level as usize - 1];

        let segment = self.generator.generate_water(
            base_segment,
            self.loaded_data.radius,
            subdivisions,
            &self.loaded_data.loaded_height,
        );

        let vertex_buffer =
            toolkit.create_vertex_buffer_from_data(segment, &WATER_ICOSPHERE_VERTEX_ATTRIBUTES)?;

        Ok(IcosphereLoadedGeometry {
            vertex_buffer,
            level,
        })
    }
}
