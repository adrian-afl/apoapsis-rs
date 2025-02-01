use crate::body::body_definitions::{load_body_data, BodyCelestialBodyDefinition, BodyTerrain};
use crate::celestial_rendering::atmosphere::atmosphere_drawer::AtmosphereDrawer;
use crate::celestial_rendering::atmosphere::clouds_generator_high_freq::CloudGeneratorHighFreq;
use crate::celestial_rendering::atmosphere::clouds_generator_low_freq::CloudGeneratorLowFreq;
use crate::celestial_rendering::buffers::celestial_body_buffer::CelestialBodyBuffer;
use crate::celestial_rendering::buffers::common_buffer::CommonBuffer;
use crate::celestial_rendering::errors::RenderingError;
use crate::celestial_rendering::finalization::multi_merger::MultiMerger;
use crate::celestial_rendering::finalization::output::Output;
use crate::celestial_rendering::geometry::g_buffer::GBuffer;
use crate::celestial_rendering::geometry::mesh_drawer::MeshDrawer;
use crate::celestial_rendering::scene::camera::Camera;
use crate::celestial_rendering::scene::material::Material;
use crate::celestial_rendering::scene::mesh::Mesh;
use crate::config::Config;
use crate::math::decimal_vector_3d::DecimalVector3d;
use crate::math::sin_cos::f64_to_dbig;
use crate::simulation::simulation::Simulation;
use crate::util::empty_textures::EMPTY_TEXTURES;
use dashu_float::DBig;
use glam::{DMat3, DMat4, DQuat, DVec3, DVec4};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tracing::{event, Level};
use vengine_rs::core::semaphore::VESemaphore;
use vengine_rs::core::toolkit::{App, VEToolkit};
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;
use winit::event::{DeviceEvent, DeviceId, KeyEvent, WindowEvent};

pub struct Renderer {
    config: Config,

    toolkit: Arc<VEToolkit>,

    pub g_buffer: GBuffer,

    pub common_buffer: CommonBuffer,

    mesh_drawer: MeshDrawer,
    cloud_generator_high_freq: CloudGeneratorHighFreq,
    cloud_generator_low_freq: CloudGeneratorLowFreq,
    atmosphere_drawer: AtmosphereDrawer,
    multi_merger: MultiMerger,
    output: Output,

    mesh_drawing_semaphore: Arc<Mutex<VESemaphore>>,
    clouds_generation_low_freq_semaphore: Arc<Mutex<VESemaphore>>,
    clouds_generation_high_freq_semaphore: Arc<Mutex<VESemaphore>>,
    atmosphere_drawing_semaphore: Arc<Mutex<VESemaphore>>,
    multi_merging_semaphore: Arc<Mutex<VESemaphore>>,
    outputting_semaphore: Arc<Mutex<VESemaphore>>,
    terrain_drawing_semaphore: Arc<Mutex<VESemaphore>>,
    water_drawing_semaphore: Arc<Mutex<VESemaphore>>,
}

impl Renderer {
    pub fn new(toolkit: Arc<VEToolkit>, config: &Config) -> Self {
        EMPTY_TEXTURES.generate(&toolkit);

        let common_buffer = CommonBuffer::new(&toolkit).expect("Failed to create CommonBuffer");

        let mut g_buffer = GBuffer::new(&config, &toolkit).expect("Failed to create G-Buffer");
        let mesh_drawer = MeshDrawer::new(&config, &toolkit, &mut g_buffer, &common_buffer)
            .expect("Failed to create MeshDrawer");

        let mut multi_merger =
            MultiMerger::new(&config, &toolkit).expect("Failed to create MultiMerger");

        let output =
            Output::new(&config, &mut multi_merger, &toolkit).expect("Failed to create Output");

        let mut cloud_generator_low_freq =
            CloudGeneratorLowFreq::new(&toolkit).expect("Failed to create CloudGeneratorLowFreq");
        let mut cloud_generator_high_freq =
            CloudGeneratorHighFreq::new(&toolkit).expect("Failed to create CloudGeneratorHighFreq");

        let atmosphere_drawer = AtmosphereDrawer::new(
            &config,
            &toolkit,
            &common_buffer,
            &mut cloud_generator_low_freq.low_freq_data_r,
            &mut cloud_generator_high_freq.high_freq_data_r,
            &mut g_buffer,
        )
        .expect("Failed to create AtmosphereDrawer");

        Self {
            config: config.clone(),
            toolkit: toolkit.clone(),
            output,
            atmosphere_drawer,
            multi_merger,
            cloud_generator_high_freq,
            cloud_generator_low_freq,
            mesh_drawer,
            g_buffer,
            common_buffer,

            mesh_drawing_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
            clouds_generation_low_freq_semaphore: Arc::new(Mutex::from(
                toolkit.create_semaphore().unwrap(),
            )),
            clouds_generation_high_freq_semaphore: Arc::new(Mutex::from(
                toolkit.create_semaphore().unwrap(),
            )),
            atmosphere_drawing_semaphore: Arc::new(Mutex::from(
                toolkit.create_semaphore().unwrap(),
            )),
            multi_merging_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
            outputting_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
            terrain_drawing_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
            water_drawing_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
        }
    }

    pub fn draw(
        &mut self,
        simulation: &Simulation,
        meshes: &[&Mesh],
        camera: &Camera,
        elapsed: f64,
        delta_time: f64,
    ) -> Result<(), RenderingError> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        self.common_buffer
            .update(camera, elapsed)
            .expect("Failed to update common_buffer");

        event!(Level::WARN, "Recording mesh_drawer");
        self.mesh_drawer
            .record(meshes)
            .expect("Failed to record mesh_drawer");

        let mut swapchain = self.toolkit.swapchain.lock().unwrap();

        event!(Level::WARN, "Submitting mesh_drawer");
        {
            let queue = &self
                .toolkit
                .queue
                .lock()
                .map_err(|_| RenderingError::QueueLockingFailed)?;
            self.mesh_drawer
                .render_stage
                .command_buffer
                .submit(
                    &queue,
                    vec![swapchain.blit_done_semaphore.clone()],
                    vec![self.mesh_drawing_semaphore.clone()],
                )
                .expect("Failed to draw mesh_drawer");
        }

        let closest_hierarchy = simulation.find_closest_hierarchy(&camera.position);

        let mut wait_for_semaphores = vec![self.mesh_drawing_semaphore.clone()];

        for body in closest_hierarchy {
            match &body.body.atmosphere {
                None => (),
                Some(atmosphere) => {
                    match &atmosphere.clouds {
                        None => (),
                        Some(_) => {
                            self.cloud_generator_high_freq
                                .update_buffer(
                                    DVec4::new(
                                        atmosphere.seed,
                                        atmosphere.seed, // TODO...
                                        atmosphere.seed,
                                        atmosphere.seed,
                                    ),
                                    elapsed,
                                    1.0,
                                )
                                .expect("Failed to update cloud_generator_high_freq");
                            self.cloud_generator_low_freq
                                .update_buffer(
                                    DVec4::new(
                                        atmosphere.seed,
                                        atmosphere.seed,
                                        atmosphere.seed,
                                        atmosphere.seed,
                                    ),
                                    elapsed,
                                    1.0,
                                )
                                .expect("Failed to update cloud_generator_low_freq");

                            let queue = &self
                                .toolkit
                                .queue
                                .lock()
                                .map_err(|_| RenderingError::QueueLockingFailed)?;
                            self.cloud_generator_high_freq
                                .compute_stage
                                .command_buffer
                                .submit(
                                    queue,
                                    vec![],
                                    vec![self.clouds_generation_high_freq_semaphore.clone()],
                                )
                                .expect("Failed to compute cloud_generator_high_freq");

                            self.cloud_generator_low_freq
                                .compute_stage
                                .command_buffer
                                .submit(
                                    queue,
                                    vec![],
                                    vec![self.clouds_generation_low_freq_semaphore.clone()],
                                )
                                .expect("Failed to compute cloud_generator_low_freq");
                        }
                    }
                    self.atmosphere_drawer
                        .set_celestial_buffer(
                            &body.celestial_body_buffer.lock().unwrap(),
                            &self.config,
                        )
                        .expect("Failed to set_celestial_buffer for atmosphere_drawer");
                }
            }

            event!(Level::WARN, "Submitting terrain_drawer");
            match &body.terrain_drawer {
                None => (),
                Some(drawer) => {
                    let queue = &self
                        .toolkit
                        .queue
                        .lock()
                        .map_err(|_| RenderingError::QueueLockingFailed)?;
                    drawer
                        .lock()
                        .unwrap()
                        .render_stage
                        .command_buffer
                        .submit(
                            queue,
                            wait_for_semaphores.clone(),
                            vec![self.terrain_drawing_semaphore.clone()],
                        )
                        .expect("Failed to draw terrain_drawer");
                    wait_for_semaphores = vec![self.terrain_drawing_semaphore.clone()]
                }
            }

            event!(Level::WARN, "Submitting water_drawer");
            match &body.water_drawer {
                None => (),
                Some(drawer) => {
                    let queue = &self
                        .toolkit
                        .queue
                        .lock()
                        .map_err(|_| RenderingError::QueueLockingFailed)?;
                    drawer
                        .lock()
                        .unwrap()
                        .render_stage
                        .command_buffer
                        .submit(
                            queue,
                            wait_for_semaphores.clone(),
                            vec![self.water_drawing_semaphore.clone()],
                        )
                        .expect("Failed to draw water_drawer");
                    wait_for_semaphores = vec![self.water_drawing_semaphore.clone()]
                }
            }

            event!(Level::WARN, "Submitting atmosphere");
            match &body.body.atmosphere {
                None => (),
                Some(_) => {
                    let mut semaphores = wait_for_semaphores.clone();
                    semaphores.push(self.clouds_generation_high_freq_semaphore.clone());
                    semaphores.push(self.clouds_generation_low_freq_semaphore.clone());

                    let queue = &self
                        .toolkit
                        .queue
                        .lock()
                        .map_err(|_| RenderingError::QueueLockingFailed)?;
                    self.atmosphere_drawer
                        .compute_stage
                        .command_buffer
                        .submit(
                            queue,
                            semaphores,
                            vec![self.atmosphere_drawing_semaphore.clone()],
                        )
                        .expect("Failed to compute atmosphere");
                    wait_for_semaphores = vec![self.atmosphere_drawing_semaphore.clone()]
                }
            }

            self.multi_merger
                .update_inputs(
                    &mut self.atmosphere_drawer.out_additive_rgb,
                    &mut self.atmosphere_drawer.out_alpha_rgba,
                    &self.config,
                )
                .expect("Failed to update multi_merger inputs");

            event!(Level::WARN, "Submitting multi_merger");
            let queue = &self
                .toolkit
                .queue
                .lock()
                .map_err(|_| RenderingError::QueueLockingFailed)?;
            self.multi_merger
                .compute_stage
                .command_buffer
                .submit(
                    queue,
                    wait_for_semaphores.clone(),
                    vec![self.multi_merging_semaphore.clone()],
                )
                .expect("Failed to compute multi_merger");
            wait_for_semaphores = vec![self.multi_merging_semaphore.clone()]
        }

        event!(Level::WARN, "Submitting output");
        self.output
            .update_buffer(1.0)
            .expect("Failed to update output buffer");
        {
            let queue = &self
                .toolkit
                .queue
                .lock()
                .map_err(|_| RenderingError::QueueLockingFailed)?;
            self.output
                .compute_stage
                .command_buffer
                .submit(
                    queue,
                    wait_for_semaphores.clone(),
                    vec![self.outputting_semaphore.clone()],
                )
                .expect("Failed to compute output");
        }

        event!(Level::WARN, "Submitting blit");
        swapchain
            .blit(&self.output.output, vec![self.outputting_semaphore.clone()])
            .expect("Failed to blit to swapchain");

        Ok(())
    }

    pub fn create_mesh(
        &mut self,
        geometry: VEVertexBuffer,
        material: Material,
    ) -> Result<Mesh, RenderingError> {
        Ok(Mesh::new(
            &self.toolkit,
            &mut self.mesh_drawer.mesh_set_layout,
            geometry,
            material,
        )?)
    }

    pub fn add_hierarchy_to_universe_simulation(
        &mut self,
        simulation: &mut Simulation,
        body: &BodyCelestialBodyDefinition,
    ) -> Result<i32, RenderingError> {
        Ok(simulation.add_hierarchy(
            &self.config,
            &mut self.g_buffer,
            &self.common_buffer,
            body,
            None,
        )?)
    }
}
