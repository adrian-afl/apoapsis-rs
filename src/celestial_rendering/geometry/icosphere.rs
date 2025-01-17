use crate::celestial_rendering::errors::CelestialRendererError;
use crate::math::decimal_vector_3d::DecimalVector3d;
use glam::{DMat4, DQuat, DVec3};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Read;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::render_stage::VERenderStage;
use vengine_rs::graphics::vertex_attributes::VertexAttribFormat;
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;

const LOD_LEVELS: u8 = 3; // 1, 2, 3

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
    currently_loaded: HashMap<String, LoadedGeometry>,
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
    ) -> Result<Icosphere, CelestialRendererError> {
        let path = format!("{}/metadata.ini", dir_path);

        let mut ico = Icosphere {
            currently_loaded: HashMap::new(),
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

        self.part_matrices.as_slice()
    }

    pub fn draw(
        &mut self,
        toolkit: &VEToolkit,
        stage: &VERenderStage,
    ) -> Result<(), CelestialRendererError> {
        for i in 0..self.metadata.len() {
            let m = &self.metadata[i];

            let final_matrix = self.part_matrices[i];

            let distance = (final_matrix.transform_vector3(DVec3::new(0.0, 0.0, 0.0))).length();

            let mut level = 1;
            if (distance < self.thresholds[1]) {
                level = 2;
            }
            if (distance < self.thresholds[0]) {
                level = 3;
            }

            let mapped = self.currently_loaded.get(&m.name);
            match mapped {
                None => {
                    self.currently_loaded.insert(
                        m.name.clone(),
                        LoadedGeometry {
                            vertex_buffer: self.load_geometry(&toolkit, &m.name, level)?,
                            level,
                        },
                    );
                }
                Some(mapped) => {
                    if (mapped.level != level) {
                        self.currently_loaded.insert(
                            m.name.clone(),
                            LoadedGeometry {
                                vertex_buffer: self.load_geometry(&toolkit, &m.name, level)?,
                                level,
                            },
                        );
                    } else {
                        stage.draw_instanced(&mapped.vertex_buffer, 1);
                    }
                }
            }
        }

        Ok(())
    }

    fn load_geometry(
        &self,
        toolkit: &VEToolkit,
        name: &str,
        level: u8,
    ) -> Result<VEVertexBuffer, CelestialRendererError> {
        let path = format!("{}/{name}.l${level}.raw", self.dir_path);
        let file = File::open(path)?;
        let mut brotli_stream = brotli::Decompressor::new(file, 40960);
        let mut decompressed = vec![];
        brotli_stream.read_to_end(&mut decompressed)?;
        Ok(toolkit.create_vertex_buffer_from_data(decompressed, &self.vertex_attributes)?)
    }
}
