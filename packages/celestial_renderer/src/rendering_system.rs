use crate::geometry::mesh_drawer::MESH_DRAWER_VERTEX_ATTRIBUTES;
use crate::renderer::Renderer;
use crate::scene::celestial_hierarchy::CelestialHierarchy;
use crate::scene::material::{ColorOrTexture, Material, ScaledTexture, ValueOrTexture};
use crate::scene::mesh::Mesh;
use ecs::component_trait::Components;
use ecs::components::rendering::mesh_component::{
    ColorOrTextureDescription, MeshDescription, ValueOrTextureDescription,
};
use ecs::ecs_world::ECSWorld;
use rayon::iter::ParallelIterator;
use renderer_common::camera::Camera;
use renderer_common::errors::RenderingError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use universe_simulation::simulation::Simulation;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::VEImageUsage;

pub struct RenderingSystem {
    toolkit: Arc<VEToolkit>,
    renderer: Mutex<Renderer>,
    celestial_hierarchy: CelestialHierarchy,
    currently_rendered_meshes: RwLock<HashMap<u64, Mesh>>,
    rendering_cutoff: f64,
}

impl RenderingSystem {
    pub fn new(toolkit: Arc<VEToolkit>, renderer: Renderer) -> Self {
        Self {
            toolkit: toolkit.clone(),
            renderer: Mutex::new(renderer),
            celestial_hierarchy: CelestialHierarchy::new(toolkit.clone()),
            currently_rendered_meshes: RwLock::new(HashMap::new()),
            rendering_cutoff: 100.0,
        }
    }

    fn create_mesh_from_description(
        &self,
        description: &MeshDescription,
    ) -> Result<Mesh, RenderingError> {
        let geometry = self.toolkit.create_vertex_buffer_from_file(
            &description.geometry_path,
            &MESH_DRAWER_VERTEX_ATTRIBUTES,
        )?;
        let material = Material {
            color: match &description.material.color {
                ColorOrTextureDescription::Color(color) => ColorOrTexture::Color(color.clone()),
                ColorOrTextureDescription::Texture(texture) => {
                    ColorOrTexture::Texture(ScaledTexture {
                        scale: texture.scale,
                        texture: self.toolkit.create_image_from_file(
                            &texture.texture_path,
                            &[VEImageUsage::Sampled],
                        )?,
                    })
                }
            },
            roughness: match &description.material.roughness {
                ValueOrTextureDescription::Value(value) => ValueOrTexture::Value(value.clone()),
                ValueOrTextureDescription::Texture(texture) => {
                    ValueOrTexture::Texture(ScaledTexture {
                        scale: texture.scale,
                        texture: self.toolkit.create_image_from_file(
                            &texture.texture_path,
                            &[VEImageUsage::Sampled],
                        )?,
                    })
                }
            },
            metalness: match &description.material.metalness {
                ValueOrTextureDescription::Value(value) => ValueOrTexture::Value(value.clone()),
                ValueOrTextureDescription::Texture(texture) => {
                    ValueOrTexture::Texture(ScaledTexture {
                        scale: texture.scale,
                        texture: self.toolkit.create_image_from_file(
                            &texture.texture_path,
                            &[VEImageUsage::Sampled],
                        )?,
                    })
                }
            },
            emission: match &description.material.emission {
                ColorOrTextureDescription::Color(color) => ColorOrTexture::Color(color.clone()),
                ColorOrTextureDescription::Texture(texture) => {
                    ColorOrTexture::Texture(ScaledTexture {
                        scale: texture.scale,
                        texture: self.toolkit.create_image_from_file(
                            &texture.texture_path,
                            &[VEImageUsage::Sampled],
                        )?,
                    })
                }
            },
            normal: match &description.material.normal {
                None => None,
                Some(texture) => Some(ScaledTexture {
                    scale: texture.scale,
                    texture: self
                        .toolkit
                        .create_image_from_file(&texture.texture_path, &[VEImageUsage::Sampled])?,
                }),
            },
            bump: match &description.material.bump {
                None => None,
                Some(texture) => Some(ScaledTexture {
                    scale: texture.scale,
                    texture: self
                        .toolkit
                        .create_image_from_file(&texture.texture_path, &[VEImageUsage::Sampled])?,
                }),
            },
        };

        self.renderer
            .lock()
            .unwrap()
            .create_mesh(geometry, material)
    }

    pub fn update(
        &mut self,
        ecs: &mut ECSWorld,
        universe_simulation: &Simulation,
        camera: &Camera,
        total_time: f64,
        delta_time: f64,
    ) {
        // println!("RenderingSystem / update");

        // this list here is so that if entity disappears, the mesh is cleaned up
        let detected_mesh_component_ids = Mutex::new(vec![]);

        let renderer_mutex = Mutex::from(&self.renderer);

        ecs.parallel_process_all_by_components(
            &[&Components::Mesh, &Components::Transform],
            |entity| {
                // println!("RenderingSystem / Parallel {}", entity.id);
                let transform_component = entity.components.transform.as_ref().unwrap();
                let mesh_components = &entity.components.mesh;

                for mesh_component in mesh_components {
                    detected_mesh_component_ids
                        .lock()
                        .unwrap()
                        .push(mesh_component.id);
                    // println!("RenderingSystem / For mesh component {}", mesh_component.id);
                    let relative_position = &transform_component.position - &camera.position;
                    let relative_position = relative_position.to_dvec3();

                    let should_render = relative_position.length() < self.rendering_cutoff;

                    let mut exists = self
                        .currently_rendered_meshes
                        .try_read()
                        .unwrap()
                        .contains_key(&mesh_component.id);

                    if !should_render && exists {
                        // println!("RenderingSystem / REMOVE {}", mesh_component.id);
                        self.currently_rendered_meshes
                            .try_write()
                            .unwrap()
                            .remove(&mesh_component.id);
                        exists = false;
                    } else if should_render && !exists {
                        // println!("RenderingSystem / ADD {}", mesh_component.id);
                        match self.create_mesh_from_description(&mesh_component.description) {
                            Err(err) => println!("Failed to create a mesh! Reason: {}", err),
                            Ok(mesh) => {
                                self.currently_rendered_meshes
                                    .try_write()
                                    .unwrap()
                                    .insert(mesh_component.id, mesh);
                                exists = true;
                            }
                        }
                    }

                    if should_render && exists {
                        // println!("RenderingSystem / UPDATE {}", mesh_component.id);
                        let mut locked_map = self.currently_rendered_meshes.try_write().unwrap();
                        let mesh = locked_map.get_mut(&mesh_component.id).unwrap();
                        mesh.position.assign(&transform_component.position);
                        mesh.scale = transform_component.scale.clone();
                        mesh.orientation = transform_component.orientation.clone();

                        mesh.update(&camera.position).unwrap();
                    }
                }
            },
        );

        // TODO this should clear up meshes that are removed from the ECS completely
        // does it work? maybe
        let mut locked_map = self.currently_rendered_meshes.try_write().unwrap();
        let detected_mesh_component_ids = detected_mesh_component_ids.lock().unwrap();
        locked_map.retain(|k, _| detected_mesh_component_ids.contains(k));
        drop(locked_map);

        // println!("RenderingSystem / After render");

        // println!("RenderingSystem / Draw");

        let render_result = self.renderer.lock().unwrap().draw(
            &self
                .currently_rendered_meshes
                .try_read()
                .unwrap()
                .values()
                .collect::<Vec<_>>(),
            universe_simulation,
            &mut self.celestial_hierarchy,
            &camera,
            total_time,
            delta_time,
        );

        // println!("RenderingSystem / End");
        match render_result {
            Ok(_) => (),
            Err(err) => println!("Render failed! Reason: {}", err),
        }
    }
}
