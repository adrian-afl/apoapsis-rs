use crate::geometry::mesh_drawer::MESH_DRAWER_VERTEX_ATTRIBUTES;
use crate::renderer::Renderer;
use crate::scene::celestial_hierarchy::CelestialHierarchy;
use crate::scene::material::{ColorOrTexture, Material, ScaledTexture, ValueOrTexture};
use crate::scene::mesh::Mesh;
use common_util::profile;
use ecs::component_trait::Components;
use ecs::components::physics::glue_to_celestial_body_component::GlueToCelestialBodyComponent;
use ecs::components::physics::real_physics_component::RealPhysicsComponent;
use ecs::components::rendering::mesh_component::{
    ColorOrTextureDescription, MeshDescription, ValueOrTextureDescription,
};
use ecs::ecs_world::ECSWorld;
use glam::DVec3;
use math::decimal_vector_3d::DecimalVector3d;
use rapier3d_f64::geometry::ColliderBuilder;
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator};
use renderer_common::camera::Camera;
use renderer_common::errors::RenderingError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use ui_renderer::ui_system::UISystem;
use universe_simulation::simulation::{SimulatedBody, Simulation};
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::VEImageUsage;

pub struct RenderingSystem {
    toolkit: Arc<VEToolkit>,
    renderer: Mutex<Renderer>,
    celestial_hierarchy: CelestialHierarchy,
    currently_rendered_meshes: RwLock<HashMap<u64, Mesh>>,
    rendering_cutoff: f64,
}

pub struct GetAltitudeResult {
    pub terrain: Option<f64>,
    pub atmosphere: Option<f64>,
    pub water: Option<f64>,
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

    pub fn recreate_stages(&mut self) -> Result<(), RenderingError> {
        self.renderer.lock().unwrap().recreate_stages()?;

        Ok(())
    }

    pub fn get_altitude(
        &self,
        body: &SimulatedBody,
        point: &DecimalVector3d,
    ) -> Option<GetAltitudeResult> {
        let rendered_body = self.celestial_hierarchy.get_rendered_body(&body.body.name);
        match rendered_body {
            None => None,
            Some(rendered_body) => {
                let mut normal = point - &body.position;
                let distance_center = normal.length();
                normal.normalize();
                let mut normal = normal.to_dvec3_with_precision(6);
                normal = (body.orientation.as_dquat() * normal).normalize();

                let terrain_altitude = rendered_body.terrain_icosphere.as_ref().map(|terrain| {
                    distance_center.to_f64().value() - terrain.get_radius_at_normal(normal)
                });
                let water_altitude = rendered_body.water_icosphere.as_ref().map(|water| {
                    distance_center.to_f64().value() - water.get_radius_at_normal(normal)
                });
                let atmosphere_altitude = body
                    .body
                    .atmosphere
                    .as_ref()
                    .map(|atmo| distance_center.to_f64().value() - atmo.start);

                Some(GetAltitudeResult {
                    water: water_altitude,
                    terrain: terrain_altitude,
                    atmosphere: atmosphere_altitude,
                })
            }
        }
    }

    pub fn get_terrain_physics_components(
        &self,
        body_name: &str,
        base_index: u16,
    ) -> Option<(
        RealPhysicsComponent,
        GlueToCelestialBodyComponent,
        ColliderBuilder,
    )> {
        let rendered_body = self.celestial_hierarchy.get_rendered_body(&body_name)?;
        rendered_body
            .terrain_icosphere
            .as_ref()?
            .get_physics_components(base_index)
    }

    pub fn get_water_physics_components(
        &self,
        body_name: &str,
        base_index: u16,
    ) -> Option<(
        RealPhysicsComponent,
        GlueToCelestialBodyComponent,
        ColliderBuilder,
    )> {
        let rendered_body = self.celestial_hierarchy.get_rendered_body(&body_name)?;
        rendered_body
            .water_icosphere
            .as_ref()?
            .get_physics_components(base_index)
    }

    pub fn should_have_physics_terrain_water(
        &self,
        body_name: &str,
        base_index: u16,
    ) -> (bool, bool) {
        let rendered_body = self.celestial_hierarchy.get_rendered_body(&body_name)?;
        (
            rendered_body
                .terrain_icosphere
                .as_ref()?
                .should_have_physics(base_index),
            rendered_body
                .water_icosphere
                .as_ref()?
                .should_have_physics(base_index),
        )
    }

    pub fn get_terrain_distance_from_center(
        &self,
        universe: &Simulation,
        body: &str,
        normal: DVec3,
    ) -> Option<f64> {
        let body = universe.get_body(body);
        let rendered_body = self.celestial_hierarchy.get_rendered_body(&body.body.name);
        match rendered_body {
            None => None,
            Some(rendered_body) => rendered_body
                .terrain_icosphere
                .as_ref()
                .map(|terrain| terrain.get_radius_at_normal(normal)),
        }
    }

    pub fn get_water_distance_from_center(
        &self,
        universe: &Simulation,
        body: &str,
        normal: DVec3,
    ) -> Option<f64> {
        let body = universe.get_body(body);
        let rendered_body = self.celestial_hierarchy.get_rendered_body(&body.body.name);
        match rendered_body {
            None => None,
            Some(rendered_body) => rendered_body
                .water_icosphere
                .as_ref()
                .map(|water| water.get_radius_at_normal(normal)),
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
                ColorOrTextureDescription::Color(color) => ColorOrTexture::Color(*color),
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
                ValueOrTextureDescription::Value(value) => ValueOrTexture::Value(*value),
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
                ValueOrTextureDescription::Value(value) => ValueOrTexture::Value(*value),
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
                ColorOrTextureDescription::Color(color) => ColorOrTexture::Color(*color),
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
        ui_system: &UISystem,
    ) {
        // println!("RenderingSystem / update");

        let existing_ids: Arc<Vec<u64>> = profile!("rendering_system update / existing_ids", {
            Arc::new(
                self.currently_rendered_meshes
                    .read()
                    .unwrap()
                    .iter()
                    .map(|(k, _v)| *k)
                    .collect(),
            )
        });

        let entities = profile!("rendering_system update / entities", {
            ecs.parallel_map_all_by_components(
                &[&Components::Mesh, &Components::Transform],
                |entity| entity.id,
            )
        });

        let mut new_meshes: Vec<(u64, Mesh)> = profile!("rendering_system update / new_meshes", {
            entities
                .par_iter()
                .filter_map(|entity| {
                    let entity = &ecs[*entity];
                    // println!("RenderingSystem / Parallel {}", entity.id);
                    let transform_component = entity.components.transform.as_ref().unwrap();
                    let mesh_components = &entity.components.mesh;

                    let existing_ids = existing_ids.clone();

                    let mut new_mesh = None;

                    for mesh_component in mesh_components {
                        let mut exists = existing_ids.contains(&mesh_component.id);

                        if !exists {
                            // println!("RenderingSystem / For mesh component {}", mesh_component.id);
                            let relative_position =
                                &transform_component.position - &camera.position;
                            let relative_position = relative_position.to_dvec3_with_precision(6);

                            if relative_position.length() < self.rendering_cutoff {
                                // println!("RenderingSystem / ADD {}", mesh_component.id);
                                match self.create_mesh_from_description(&mesh_component.description)
                                {
                                    Err(err) => {
                                        println!("Failed to create a mesh! Reason: {}", err)
                                    }
                                    Ok(mesh) => {
                                        new_mesh = Some((mesh_component.id, mesh));
                                        exists = true;
                                    }
                                }
                            }
                        }
                    }
                    new_mesh
                })
                .collect()
        });

        {
            let mut locked_map = self.currently_rendered_meshes.write().unwrap();

            profile!("rendering_system update / adding new meshes", {
                let len = new_meshes.len();
                for _ in 0..len {
                    let item = new_meshes.pop();
                    if let Some(item) = item {
                        locked_map.insert(item.0, item.1);
                    }
                }
            });

            profile!("rendering_system update / updating mesh components", {
                entities.iter().for_each(|entity| {
                    let entity = &mut ecs[*entity];
                    let transform_component = entity.components.transform.as_ref().unwrap();
                    entity.components.mesh.iter().for_each(|mc| {
                        let mesh = locked_map.get_mut(&mc.id).unwrap();
                        mesh.position.assign(&transform_component.position);
                        mesh.scale = transform_component.scale;
                        mesh.orientation = transform_component.orientation;
                    });
                });
            });
        }

        let detected_mesh_component_ids: Vec<u64> =
            profile!("rendering_system update / detected_mesh_component_ids", {
                entities
                    .par_iter()
                    .map(|entity| {
                        let entity = &ecs[*entity];
                        entity
                            .components
                            .mesh
                            .iter()
                            .map(|mc| mc.id)
                            .collect::<Vec<u64>>()
                    })
                    .flatten()
                    .collect()
            });

        // TODO this should clear up meshes that are removed from the ECS completely
        // does it work? maybe
        // update it turns out to be working
        {
            let mut locked_map = self.currently_rendered_meshes.write().unwrap();

            profile!("rendering_system update / mesh update retain", {
                locked_map.retain(|k, _| detected_mesh_component_ids.contains(k));
            });

            profile!("rendering_system update / mesh update pass 2", {
                locked_map.par_iter_mut().for_each(|(_, mesh)| {
                    mesh.update(&camera.position).unwrap();
                });
            });
        }
        // println!("RenderingSystem / After render");

        // println!("RenderingSystem / Draw");

        ui_system.with_items(|items| {
            let render_result = self.renderer.lock().unwrap().draw(
                &self
                    .currently_rendered_meshes
                    .read()
                    .unwrap()
                    .values()
                    .collect::<Vec<_>>(),
                items,
                universe_simulation,
                &mut self.celestial_hierarchy,
                camera,
                &ecs.time_counter,
            );

            // println!("RenderingSystem / End");
            match render_result {
                Ok(_) => (),
                Err(err) => println!("Render failed! Reason: {}", err),
            }
        });
    }
}
