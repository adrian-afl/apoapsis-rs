use crate::buffers::celestial_body_buffer::CelestialBodyBuffer;
use crate::geometry::terrain_icosphere::TerrainIcosphere;
use crate::geometry::terrain_icosphere_drawer::TerrainIcosphereDrawer;
use crate::geometry::water_icosphere::WaterIcosphere;
use crate::geometry::water_icosphere_drawer::WaterIcosphereDrawer;
use glam::DVec3;
use math::decimal_vector_3d::DecimalVector3d;
use renderer_common::errors::RenderingError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use universe_simulation::body_definitions::BodyCelestialBodyDefinition;
use universe_simulation::simulation::Simulation;
use vengine_rs::core::toolkit::VEToolkit;

pub struct RenderedBody {
    pub body: BodyCelestialBodyDefinition,
    pub terrain: Option<TerrainIcosphere>,
    pub water: Option<WaterIcosphere>,
    pub celestial_body_buffer: CelestialBodyBuffer,
}

pub struct CelestialHierarchy {
    toolkit: Arc<VEToolkit>,
    universe_simulation: Arc<RwLock<Simulation>>,
    rendered_bodies: HashMap<String, RenderedBody>,
}

impl CelestialHierarchy {
    pub fn new(toolkit: Arc<VEToolkit>, universe_simulation: Arc<RwLock<Simulation>>) -> Self {
        Self {
            toolkit,
            universe_simulation,
            rendered_bodies: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        camera_position: &DecimalVector3d,
        terrain_icosphere_drawer: &mut TerrainIcosphereDrawer,
        water_icosphere_drawer: &mut WaterIcosphereDrawer,
    ) -> Result<(), RenderingError> {
        let sim = self.universe_simulation.try_read().unwrap();
        let closest_hierarchy = sim.find_closest_hierarchy(&camera_position);

        let closest_star = sim.find_closest_static(&camera_position);

        let mut found_names = vec![];
        for closest_hierarchy_body in closest_hierarchy {
            found_names.push(closest_hierarchy_body.body.name.clone());
            let mut exists = self
                .rendered_bodies
                .contains_key(&closest_hierarchy_body.body.name);
            if !exists {
                let terrain_ico = match &closest_hierarchy_body.body.terrain {
                    None => None,
                    Some(terrain) => Some(TerrainIcosphere::new(
                        &self.toolkit,
                        &mut terrain_icosphere_drawer.data_set_layout,
                        terrain.icosphere_path.to_owned(),
                        vec![2000000.0, 5000000.0],
                    )?),
                };

                let water_ico = match &closest_hierarchy_body.body.water {
                    None => None,
                    Some(water) => Some(WaterIcosphere::new(
                        &self.toolkit,
                        &mut water_icosphere_drawer.data_set_layout,
                        water.icosphere_path.to_owned(),
                        vec![2000000.0, 5000000.0],
                        water.color,
                    )?),
                };

                let buffer = CelestialBodyBuffer::new(&self.toolkit)?;

                self.rendered_bodies.insert(
                    closest_hierarchy_body.body.name.clone(),
                    RenderedBody {
                        body: closest_hierarchy_body.body.clone(),
                        terrain: terrain_ico,
                        water: water_ico,
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

                if let Some(ref mut terrain) = &mut body.terrain {
                    terrain.update_buffer(&camera_position, &closest_hierarchy_body)?;
                }

                if let Some(ref mut water) = &mut body.water {
                    water.update_buffer(&camera_position, &closest_hierarchy_body)?;
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
