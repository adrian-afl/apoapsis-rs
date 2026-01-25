use crate::buffers::terrain_icosphere_data_buffer::TerrainIcosphereDataBuffer;
use crate::geometry::common_icosphere::{
    ICO_LEVEL_SUBDIVISIONS, IcosphereLoadedGeometry, PreloadDetectionResultAction, PreloadResult,
    get_icosphere_segment_displacement, update_icosphere_matrices, which_part_to_preload,
};
use crate::geometry::icosphere_drawer::TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES;
use ecs::component_trait::acquire_next_id;
use ecs::components::physics::glue_to_celestial_body_component::GlueToCelestialBodyComponent;
use ecs::components::physics::real_physics_component::{
    CelestialBodyColliderSurfaceType, CelestialBodySurfaceColliderDescription,
    RealPhysicsComponent, ShapeDescription, TriMeshColliderDescription,
};
use glam::{DMat4, DQuat, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use planet_generator_library::cubemap_data::CubeMapDataLayer;
use planet_generator_library::generate_icosphere::{
    IcosphereMetadataItem, IcosphereSegmentGenerator, Triangle, generate_icosphere_metadata,
};
use planet_generator_library::interpolated_biome_data::LoadedBiomeData;
use planet_generator_library::load_binary_maps::{
    get_terrain_maps_resolution, load_binary_biome_map, load_binary_terrain_map,
};
use rapier3d_f64::math::Vector;
use rapier3d_f64::prelude::{ColliderBuilder, TriMeshFlags};
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
    body_name: String,
    radius: f64,
    loaded_height: CubeMapDataLayer<f64>,
    loaded_biome: CubeMapDataLayer<LoadedBiomeData>,
    metadata: Vec<IcosphereMetadataItem>,
    part_matrices: Vec<DMat4>,
}

pub struct TerrainIcosphere {
    generator: Arc<IcosphereSegmentGenerator>,
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
        body_name: &str,
        terrain_data: TerrainData,
        base_icosphere: Arc<Vec<Triangle>>,
        data_set_layout: &mut VEDescriptorSetLayout,
    ) -> Result<TerrainIcosphere, RenderingError> {
        let metadata = generate_icosphere_metadata(&base_icosphere, terrain_data.radius);
        let part_matrices = vec![DMat4::IDENTITY; metadata.len()];
        let maps_resolutions = get_terrain_maps_resolution(&terrain_data.dir_path);
        let loaded_data = LoadedTerrainData {
            body_name: body_name.to_owned(),
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

    pub fn get_physics_components(
        &self,
        base_index: u16,
    ) -> Option<(
        RealPhysicsComponent,
        GlueToCelestialBodyComponent,
        ColliderBuilder,
    )> {
        let locked = self.currently_loaded.lock().unwrap();
        let loaded = locked.get(&base_index);
        match loaded {
            None => None,
            Some(loaded) => {
                if loaded.collider_builder.is_none() {
                    return None;
                }
                Some((
                    loaded.real_physics_component.as_ref().unwrap().clone(),
                    loaded
                        .glue_to_celestial_body_component
                        .as_ref()
                        .unwrap()
                        .clone(),
                    loaded.collider_builder.as_ref().unwrap().clone(),
                ))
            }
        }
    }

    pub fn should_have_physics(&self, base_index: u16) -> bool {
        let locked = self.currently_loaded.lock().unwrap();
        let loaded = locked.get(&base_index);
        match loaded {
            None => false,
            Some(loaded) => loaded.collider_builder.is_some(),
        }
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

    /*
    How the physics should be loaded

    There are 2 processess going on, physics in general working every frame, and this preload thing
    This is probably good idea to only enable physics at the most detailed LOD level

    The problem is this streaming of LOD levels is happening every frame and stuff can get loaded and unloaded at any moment

    so there are 2 options, 2 directions:

        1. Let the renderer tell physics when to move the bodies
            - This can reuse existing ECS infrastructure
            - would need to check every frame for preloads that are loaded with highest detail level
            - in case new one appears, its added to ECS with a reference to renderer collider - not simple to pull it from there

            Risks:
            - Getting collider data from renderer to physics when the collider is being built is going to be weird
            - Maybe caching can be added but its another layer of bullshit
            - It looks like the systems should be decoupled in a way to offer similar api to the one TS uses
                - for example, emitting an event to the whole game to manage the colliders etc, and get data too
                - like a command to get the collider data and something responsds to it
                - it might be increasibly likely that handling this on TS side will just be easier as its already decoupled

        2. Let the physics system check for actions to do based on what renderer is rendering
            - Not really ECS reuse but can be done as well
     */

    fn load_geometry(
        &self,
        toolkit: &VEToolkit,
        base_segment: u16,
        level: u8,
    ) -> Result<IcosphereLoadedGeometry, RenderingError> {
        let subdivisions = ICO_LEVEL_SUBDIVISIONS[level as usize - 1];

        let is_most_detailed_level = level as usize == ICO_LEVEL_SUBDIVISIONS.len(); // 1, 2, 3, len == 3 so len must eq 3

        let segment = self.generator.generate_terrain(
            base_segment,
            self.loaded_data.radius,
            subdivisions,
            &self.loaded_data.loaded_height,
            &self.loaded_data.loaded_biome,
        );

        let vertex_buffer = toolkit
            .create_vertex_buffer_from_data(segment.0, &TERRAIN_ICOSPHERE_VERTEX_ATTRIBUTES)?;

        let vertices = segment.1;
        let mut indices: Vec<[u32; 3]> = Vec::new();
        let mut i = 0;
        for vertex_i in 0..vertices.len() / 3 {
            indices.push([(i + 0) as u32, (i + 1) as u32, (i + 2) as u32]);
            i += 3;
        }

        Ok(IcosphereLoadedGeometry {
            vertex_buffer,
            real_physics_component: if !is_most_detailed_level {
                None
            } else {
                Some(RealPhysicsComponent {
                    id: acquire_next_id(),
                    shape_description: ShapeDescription::CelestialBodySurface(
                        CelestialBodySurfaceColliderDescription {
                            body_name: self.loaded_data.body_name.clone(),
                            surface_type: CelestialBodyColliderSurfaceType::Terrain,
                            index: base_segment,
                        },
                    ),
                    override_real_simulation_cutoff: Some(self.loaded_data.radius * 0.23),
                })
            },
            glue_to_celestial_body_component: if !is_most_detailed_level {
                None
            } else {
                Some(GlueToCelestialBodyComponent {
                    body_name: self.loaded_data.body_name.clone(),
                    id: acquire_next_id(),
                    offset: get_icosphere_segment_displacement(
                        &self.loaded_data.metadata,
                        base_segment as usize,
                    ),
                    orientation: DQuat::IDENTITY,
                })
            },
            collider_builder: if !is_most_detailed_level {
                None
            } else {
                Some(
                    ColliderBuilder::trimesh_with_flags(
                        vertices
                            .iter()
                            .map(|x| Vector::new(x.x, x.y, x.z))
                            .collect(),
                        indices,
                        TriMeshFlags::FIX_INTERNAL_EDGES,
                    )
                    .unwrap()
                    .contact_skin(1.0)
                    .friction(0.2),
                )
            },
            level,
        })
    }
}
