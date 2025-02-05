use crate::buffers::celestial_body_buffer::CelestialBodyBuffer;
use crate::geometry::icosphere::Icosphere;
use crate::geometry::icosphere_drawer::IcosphereDrawer;
use glam::DVec3;
use math::decimal_vector_3d::DecimalVector3d;
use planet_generator_library::generate_icosphere::{generate_base_icosphere, Triangle};
use renderer_common::errors::RenderingError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use universe_simulation::body_definitions::BodyCelestialBodyDefinition;
use universe_simulation::simulation::Simulation;
use vengine_rs::core::toolkit::VEToolkit;

pub struct RenderedBody {
    pub body: BodyCelestialBodyDefinition,
    pub icosphere: Option<Icosphere>,
    pub celestial_body_buffer: CelestialBodyBuffer,
}

pub struct CelestialHierarchy {
    toolkit: Arc<VEToolkit>,
    rendered_bodies: HashMap<String, RenderedBody>,
    base_icosphere: Arc<Vec<Triangle>>,
}

impl CelestialHierarchy {
    pub fn new(toolkit: Arc<VEToolkit>) -> Self {
        Self {
            toolkit,
            rendered_bodies: HashMap::new(),
            base_icosphere: Arc::new(generate_base_icosphere(2)),
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
                let icosphere = match &closest_hierarchy_body.body.terrain {
                    None => None,
                    Some(terrain) => Some(Icosphere::new(
                        &self.toolkit,
                        terrain.radius,
                        terrain.icosphere_path.to_owned(),
                        vec![2000000.0, 5000000.0],
                        self.base_icosphere.clone(),
                        match &closest_hierarchy_body.body.water {
                            None => None,
                            Some(water) => Some(water.color),
                        },
                        &mut icosphere_drawer.data_set_layout,
                    )?),
                };

                let buffer = CelestialBodyBuffer::new(&self.toolkit)?;

                self.rendered_bodies.insert(
                    closest_hierarchy_body.body.name.clone(),
                    RenderedBody {
                        body: closest_hierarchy_body.body.clone(),
                        icosphere,
                        celestial_body_buffer: buffer,
                    },
                );
                exists = true;
            }

            if exists {
                let mut body = self
                    .rendered_bodies
                    .get_mut(&closest_hierarchy_body.body.name)
                    .unwrap();
                // println!("{:?}", closest_star.body);
                body.celestial_body_buffer.update(
                    &camera_position,
                    &closest_star.position,
                    match &closest_star.body.star_emission {
                        None => DVec3::new(0.0, 0.0, 0.0),
                        Some(radiance) => radiance.radiance,
                    },
                    &closest_hierarchy_body,
                )?;

                if let Some(ref mut icosphere) = &mut body.icosphere {
                    icosphere.update_buffer(&camera_position, &closest_hierarchy_body)?;
                }
            }
        }

        // AMAZING finally it looks better, hope it works
        self.rendered_bodies.retain(|k, _| found_names.contains(&k));

        Ok(())
    }

    pub fn get_rendered_bodies(&mut self) -> Vec<&mut RenderedBody> {
        // TODO here sorting is needed but its not really possible yet, it needs some work
        let mut refs = vec![];
        for body in self.rendered_bodies.values_mut() {
            refs.push(body);
        }
        refs
    }
}
