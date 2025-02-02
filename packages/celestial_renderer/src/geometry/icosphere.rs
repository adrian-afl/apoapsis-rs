use glam::{DMat4, DQuat, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator};
use renderer_common::errors::RenderingError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::render_stage::VERenderStage;
use vengine_rs::graphics::vertex_attributes::VertexAttribFormat;
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;

// const LOD_LEVELS: u8 = 3; // 1, 2, 3

struct LoadedGeometry {
    pub vertex_buffer: VEVertexBuffer,
    pub level: u8,
}

struct MetadataItem {
    pub name: String,
    pub center: DVec3,
    pub global_index: u32,
}

pub struct Icosphere {
    currently_loaded: Mutex<HashMap<String, LoadedGeometry>>,
    metadata: Vec<MetadataItem>,
    dir_path: String,
    thresholds: Vec<f64>,
    vertex_attributes: Vec<VertexAttribFormat>,
    part_matrices: Vec<DMat4>,
}

impl Icosphere {
    pub fn new(
        dir_path: String,
        thresholds: Vec<f64>,
        vertex_attributes: Vec<VertexAttribFormat>,
    ) -> Result<Icosphere, RenderingError> {
        let path = format!("{}/metadata.ini", dir_path);

        let mut ico = Icosphere {
            currently_loaded: Mutex::new(HashMap::new()),
            metadata: vec![],
            dir_path,
            thresholds,
            vertex_attributes,
            part_matrices: vec![],
        };

        for line in read_to_string(path)?.lines() {
            let split: Vec<&str> = line.split("=").collect();
            if split.len() != 2 {
                println!("weird line encountered in metadata, whole line is weird: {line}");
                continue;
            }
            let name = split[0];
            let index_parts: Vec<u32> = name
                .split("-")
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .collect();
            let global_index = index_parts[0] * 16 + index_parts[1]; // ????
            let vector_str_parts: Vec<&str> = split[1].split(",").collect();
            if vector_str_parts.len() != 3 {
                println!("weird line encountered in metadata, split 1 is weird: {line}");
                continue;
            }
            let vector = DVec3::new(
                vector_str_parts[0].parse::<f64>().unwrap_or_default(),
                vector_str_parts[1].parse::<f64>().unwrap_or_default(),
                vector_str_parts[2].parse::<f64>().unwrap_or_default(),
            );

            ico.metadata.push(MetadataItem {
                name: name.to_owned(),
                center: vector,
                global_index,
            });
            ico.part_matrices.push(DMat4::IDENTITY.clone());
        }

        ico.metadata
            .sort_by(|a, b| a.global_index.partial_cmp(&b.global_index).unwrap());

        Ok(ico)
    }

    pub fn update_and_get_part_matrices<'a>(
        &'a mut self,
        camera_position: &DecimalVector3d,
        sphere_position: &DecimalVector3d,
        sphere_orientation: DQuat,
    ) -> &'a [DMat4] {
        let relative_camera_position = sphere_position - camera_position;

        let rotation_matrix = DMat4::from_quat(sphere_orientation).inverse();
        let world_translation_matrix = DMat4::from_translation(relative_camera_position.to_dvec3());
        let pre_final_matrix = world_translation_matrix * rotation_matrix;

        for i in 0..self.metadata.len() {
            let metadata = &self.metadata[i];
            let model_offset_matrix = DMat4::from_translation(metadata.center);
            self.part_matrices[i] = pre_final_matrix * model_offset_matrix;
        }

        println!("FD {}", relative_camera_position);
        self.part_matrices.as_slice()
    }

    pub fn draw(
        &mut self,
        toolkit: &VEToolkit,
        stage: &VERenderStage,
    ) -> Result<(), RenderingError> {
        let lock = Mutex::new(());
        self.metadata.par_iter().enumerate().for_each(|(i, m)| {
            let m = &self.metadata[i];

            let final_matrix = self.part_matrices[i];

            let distance = (final_matrix.transform_vector3(DVec3::new(0.0, 0.0, 0.0))).length();

            // println!("distance {distance}");

            let mut level = 1;
            if (distance < self.thresholds[1]) {
                level = 2;
            }
            if (distance < self.thresholds[0]) {
                level = 3;
            }

            let exists = self.currently_loaded.lock().unwrap().contains_key(&m.name);
            if exists {
                let mapped_level = self
                    .currently_loaded
                    .lock()
                    .unwrap()
                    .get(&m.name)
                    .unwrap()
                    .level;
                if (mapped_level != level) {
                    let geometry = self.load_geometry(&toolkit, &lock, &m.name, level).unwrap();
                    let mut locked = self.currently_loaded.lock().unwrap();
                    let mut mapped_mut = locked.get_mut(&m.name).unwrap();
                    mapped_mut.level = level;
                    mapped_mut.vertex_buffer = geometry;
                } else {
                    let locked = self.currently_loaded.lock().unwrap();
                    let mapped = locked.get(&m.name).unwrap();
                    stage.draw_instanced(&mapped.vertex_buffer, 1);
                }
            } else {
                let geometry = self.load_geometry(&toolkit, &lock, &m.name, level).unwrap();
                self.currently_loaded.lock().unwrap().insert(
                    m.name.clone(),
                    LoadedGeometry {
                        vertex_buffer: geometry,
                        level,
                    },
                );
            }
        });

        Ok(())
    }

    fn load_geometry(
        &self,
        toolkit: &VEToolkit,
        lock: &Mutex<()>,
        name: &str,
        level: u8,
    ) -> Result<VEVertexBuffer, RenderingError> {
        let path = format!("{}/{name}.l{level}.raw", self.dir_path);
        //println!("LOADING {}", path);
        let file = File::open(path)?;
        let mut brotli_stream = brotli::Decompressor::new(file, 40960);
        let mut decompressed = vec![];
        brotli_stream.read_to_end(&mut decompressed)?;
        Ok({
            let _exclusivity_lock = lock.lock().unwrap(); // this probably could be done better if i did it in vengine
            toolkit.create_vertex_buffer_from_data(decompressed, &self.vertex_attributes)?
        })
    }
}
