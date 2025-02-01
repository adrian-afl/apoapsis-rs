use crate::celestial_rendering::errors::RenderingError;
use crate::celestial_rendering::geometry::mesh_drawer::MESH_DRAWER_VERTEX_ATTRIBUTES;
use crate::celestial_rendering::renderer::Renderer;
use crate::celestial_rendering::scene::material::{
    ColorOrTexture, Material, ScaledTexture, ValueOrTexture,
};
use crate::celestial_rendering::scene::mesh::Mesh;
use crate::component_types;
use crate::core::game_state::GameState;
use crate::ecs::component_trait::component_type;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::ecs_components::rendering::mesh_component::{
    ColorOrTextureDescription, MeshComponent, MeshDescription, ValueOrTextureDescription,
};
use crate::simulation::simulation::Simulation;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::image::image::VEImageUsage;

pub struct RenderingSystem {
    toolkit: Arc<VEToolkit>,
    renderer: Arc<Mutex<Renderer>>,
    universe_simulation: Arc<Mutex<Simulation>>,
    currently_rendered_meshes: Arc<Mutex<HashMap<u64, Mesh>>>,
    rendering_cutoff: f64,
}

impl RenderingSystem {
    pub fn new(
        toolkit: Arc<VEToolkit>,
        renderer: Arc<Mutex<Renderer>>,
        universe_simulation: Arc<Mutex<Simulation>>,
    ) -> Self {
        Self {
            toolkit,
            renderer,
            universe_simulation,
            currently_rendered_meshes: Arc::new(Mutex::from(HashMap::new())),
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
}

impl SystemTrait for RenderingSystem {
    fn update(&mut self, game_state: Arc<Mutex<GameState>>, ecs: Arc<Mutex<ECSWorld>>) {
        let ecs = ecs.lock().unwrap();
        ecs.process_all_by_components(
            component_types!(MeshComponent, TransformComponent),
            |entity| {
                let transform_component =
                    entity.get_first_component::<TransformComponent>().unwrap();
                let mesh_component = entity.get_first_component::<MeshComponent>().unwrap();

                let relative_position = &transform_component.position
                    - &game_state.lock().unwrap().current_camera.position;
                let relative_position = relative_position.to_dvec3();

                let should_render = relative_position.length() < self.rendering_cutoff;
                let mut exists = self
                    .currently_rendered_meshes
                    .lock()
                    .unwrap()
                    .contains_key(&mesh_component.id);

                if !should_render && exists {
                    self.currently_rendered_meshes
                        .lock()
                        .unwrap()
                        .remove(&mesh_component.id);
                    exists = false;
                } else if should_render && !exists {
                    match self.create_mesh_from_description(&mesh_component.description) {
                        Err(err) => println!("Failed to create a mesh! Reason: {}", err),
                        Ok(mesh) => {
                            self.currently_rendered_meshes
                                .lock()
                                .unwrap()
                                .insert(mesh_component.id, mesh);
                            exists = true;
                        }
                    }
                }

                if should_render && exists {
                    let mut map_locked = self.currently_rendered_meshes.lock().unwrap();
                    let mut mesh = map_locked.get_mut(&mesh_component.id).unwrap();
                    mesh.position.assign(&transform_component.position);
                    mesh.scale = transform_component.scale.clone();
                    mesh.orientation = transform_component.orientation.clone();

                    mesh.update(&game_state.lock().unwrap().current_camera.position)
                        .unwrap();
                }
            },
        );

        let locked_state = &game_state.lock().unwrap();

        let render_result = self.renderer.lock().unwrap().draw(
            &self.universe_simulation.lock().unwrap(),
            &self
                .currently_rendered_meshes
                .lock()
                .unwrap()
                .values()
                .collect::<Vec<_>>(),
            &locked_state.current_camera,
            locked_state.elapsed,
            locked_state.delta_time,
        );

        match render_result {
            Ok(_) => (),
            Err(err) => println!("Render failed! Reason: {}", err),
        }
    }
}
