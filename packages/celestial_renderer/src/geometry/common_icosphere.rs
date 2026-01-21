use ecs::components::physics::glue_to_celestial_body_component::GlueToCelestialBodyComponent;
use ecs::components::physics::real_physics_component::RealPhysicsComponent;
use glam::{DMat4, DVec3, Vec3};
use math::decimal_vector_3d::DecimalVector3d;
use planet_generator_library::generate_icosphere::IcosphereMetadataItem;
use rapier3d_f64::geometry::ColliderBuilder;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::HashMap;
use std::sync::Mutex;
use universe_simulation::simulation::SimulatedBody;
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;

pub static ICO_LEVEL_SUBDIVISIONS: [u8; 3] = [3, 4, 5];
static ICO_THRESHOLDS: [f64; 2] = [2000000.0, 5000000.0];

pub struct IcosphereLoadedGeometry {
    pub vertex_buffer: VEVertexBuffer,
    pub collider_builder: Option<ColliderBuilder>,
    pub real_physics_component: Option<RealPhysicsComponent>,
    pub glue_to_celestial_body_component: Option<GlueToCelestialBodyComponent>,
    pub level: u8,
}

#[derive(PartialEq, Eq)]
pub enum PreloadDetectionResultAction {
    Insert,
    Update,
}

pub struct PreloadDetectionResult {
    pub base_segment: u16,
    pub level: u8,
    pub action: PreloadDetectionResultAction,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum PreloadResult {
    ChangesMade,
    NotChanged,
}

pub fn which_part_to_preload(
    metadata: &Vec<IcosphereMetadataItem>,
    part_matrices: &Vec<DMat4>,
    currently_loaded: &HashMap<u16, IcosphereLoadedGeometry>,
) -> Vec<PreloadDetectionResult> {
    let result = Mutex::new(vec![]);

    metadata.par_iter().enumerate().for_each(|(i, m)| {
        let final_matrix = part_matrices[i];

        let distance = final_matrix
            .transform_vector3(DVec3::new(0.0, 0.0, 0.0))
            .length();

        // println!("distance {distance}");

        let mut level = 1;
        if distance < ICO_THRESHOLDS[1] {
            level = 2;
        }
        if distance < ICO_THRESHOLDS[0] {
            level = 3;
        }

        let exists = currently_loaded.contains_key(&m.base_segment);
        if exists {
            let mapped_level = currently_loaded.get(&m.base_segment).unwrap().level;
            if mapped_level != level {
                result.lock().unwrap().push(PreloadDetectionResult {
                    base_segment: m.base_segment,
                    level,
                    action: PreloadDetectionResultAction::Update,
                });
            }
        } else {
            result.lock().unwrap().push(PreloadDetectionResult {
                base_segment: m.base_segment,
                level,
                action: PreloadDetectionResultAction::Insert,
            });
        }
    });

    result.into_inner().unwrap()
}

pub fn update_icosphere_matrices(
    camera_position: &DecimalVector3d,
    simulated_body: &SimulatedBody,
    metadata: &Vec<IcosphereMetadataItem>,
    part_matrices: &mut Vec<DMat4>,
) {
    let relative_camera_position = &simulated_body.position - camera_position;

    let rotation_matrix = simulated_body.orientation.as_dmat4().inverse();
    let world_translation_matrix =
        DMat4::from_translation(relative_camera_position.to_dvec3_with_precision(6));
    let pre_final_matrix = world_translation_matrix * rotation_matrix;

    for i in 0..metadata.len() {
        let metadata = &metadata[i];
        let model_offset_matrix = DMat4::from_translation(metadata.center);
        part_matrices[i] = pre_final_matrix * model_offset_matrix;
    }
}

pub fn get_icosphere_segment_displacement(
    metadata: &Vec<IcosphereMetadataItem>,
    index: usize,
) -> DVec3 {
    let metadata = &metadata[index];
    metadata.center
}
