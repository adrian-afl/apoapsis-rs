use crate::buffers::celestial_body_buffer::CelestialBodyBuffer;
use crate::geometry::common_icosphere::ICO_LEVEL_SUBDIVISIONS;
use crate::geometry::icosphere_drawer::IcosphereDrawer;
use crate::geometry::terrain_icosphere::{TerrainData, TerrainIcosphere};
use crate::geometry::water_icosphere::{WaterData, WaterIcosphere};
use common_util::{profile, udebug};
use dashu_float::DBig;
use glam::DVec3;
use math::decimal_vector_3d::DecimalVector3d;
use planet_generator_library::generate_icosphere::{
    generate_base_icosphere, IcosphereSegmentGenerator, Triangle,
};
use rayon::join;
use renderer_common::errors::RenderingError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use universe_simulation::body_definitions::BodyCelestialBodyDefinition;
use universe_simulation::simulation::Simulation;
use vengine_rs::core::toolkit::VEToolkit;

pub struct RenderedBody {
    pub body: BodyCelestialBodyDefinition,
    pub terrain_icosphere: Option<TerrainIcosphere>,
    pub water_icosphere: Option<WaterIcosphere>,
    pub celestial_body_buffer: CelestialBodyBuffer,
    distance_to_camera: DBig,
}

pub struct CelestialHierarchy {
    toolkit: Arc<VEToolkit>,
    generator: Arc<IcosphereSegmentGenerator>,
    rendered_bodies: HashMap<String, RenderedBody>,
    base_icosphere: Arc<Vec<Triangle>>,
}

impl CelestialHierarchy {
    pub fn new(toolkit: Arc<VEToolkit>) -> Self {
        let base_icosphere = Arc::new(generate_base_icosphere(2));
        let generator = Arc::new(IcosphereSegmentGenerator::new(
            &base_icosphere,
            &ICO_LEVEL_SUBDIVISIONS,
        ));
        Self {
            toolkit,
            generator,
            rendered_bodies: HashMap::new(),
            base_icosphere,
        }
    }

    pub fn update(
        &mut self,
        universe_simulation: &Simulation,
        camera_position: &DecimalVector3d,
        icosphere_drawer: &mut IcosphereDrawer,
    ) -> Result<(), RenderingError> {
        let closest_hierarchy = universe_simulation.find_closest_hierarchy(&camera_position);

        let closest_star = universe_simulation.find_closest_static(&camera_position);

        let mut found_names = vec![];
        for closest_hierarchy_body in closest_hierarchy {
            found_names.push(closest_hierarchy_body.body.name.clone());
            let mut exists = self
                .rendered_bodies
                .contains_key(&closest_hierarchy_body.body.name);
            if !exists {
                udebug!(
                    "Adding body {} because it didnt exist in rendered bodies",
                    &closest_hierarchy_body.body.name
                );
                let terrain_icosphere = match &closest_hierarchy_body.body.terrain {
                    None => None,
                    Some(terrain) => Some(TerrainIcosphere::new(
                        &self.toolkit,
                        self.generator.clone(),
                        TerrainData {
                            radius: terrain.radius,
                            dir_path: terrain.icosphere_path.clone(),
                        },
                        self.base_icosphere.clone(),
                        &mut icosphere_drawer.data_set_layout,
                    )?),
                };
                let water_icosphere = match &closest_hierarchy_body.body.water {
                    None => None,
                    Some(water) => Some(WaterIcosphere::new(
                        &self.toolkit,
                        self.generator.clone(),
                        WaterData {
                            radius: water.radius,
                            water_color: water.color,
                            dir_path: water.icosphere_path.clone(),
                        },
                        self.base_icosphere.clone(),
                        &mut icosphere_drawer.data_set_layout,
                    )?),
                };

                let buffer = CelestialBodyBuffer::new(&self.toolkit)?;

                self.rendered_bodies.insert(
                    closest_hierarchy_body.body.name.clone(),
                    RenderedBody {
                        body: closest_hierarchy_body.body.clone(),
                        terrain_icosphere,
                        water_icosphere,
                        celestial_body_buffer: buffer,
                        distance_to_camera: camera_position
                            .distance_to(&closest_hierarchy_body.position),
                    },
                );
                exists = true;
            }

            if exists {
                udebug!("Updating body {}", &closest_hierarchy_body.body.name);
                let mut body = self
                    .rendered_bodies
                    .get_mut(&closest_hierarchy_body.body.name)
                    .unwrap();

                join(
                    || {
                        profile!("update the buffer", {
                            body.distance_to_camera =
                                camera_position.distance_to(&closest_hierarchy_body.position);
                            // println!("{:?}", closest_star.body);
                            body.celestial_body_buffer
                                .update(
                                    &camera_position,
                                    &closest_star.position,
                                    match &closest_star.body.star_emission {
                                        None => DVec3::new(0.0, 0.0, 0.0),
                                        Some(radiance) => radiance.radiance,
                                    },
                                    &closest_hierarchy_body,
                                )
                                .unwrap();
                        });
                    },
                    || {
                        join(
                            || {
                                profile!("update terrain_icosphere", {
                                    if let Some(ref mut icosphere) = &mut body.terrain_icosphere {
                                        icosphere
                                            .update_buffer(
                                                &camera_position,
                                                &closest_hierarchy_body,
                                            )
                                            .unwrap();
                                    }
                                });
                            },
                            || {
                                profile!("update water_icosphere", {
                                    if let Some(ref mut icosphere) = &mut body.water_icosphere {
                                        icosphere
                                            .update_buffer(
                                                &camera_position,
                                                &closest_hierarchy_body,
                                            )
                                            .unwrap();
                                    }
                                });
                            },
                        );
                    },
                );
            }
        }

        // AMAZING finally it looks better, hope it works
        self.rendered_bodies.retain(|k, _| found_names.contains(&k));

        Ok(())
    }

    pub fn get_rendered_body(&self, name: &str) -> Option<&RenderedBody> {
        self.rendered_bodies.get(name)
    }

    pub fn get_rendered_bodies(&mut self) -> Vec<&mut RenderedBody> {
        let mut refs = vec![];
        for body in self.rendered_bodies.values_mut() {
            refs.push(body);
        }
        refs.sort_by(|a, b| a.distance_to_camera.cmp(&b.distance_to_camera));
        refs
    }
}
