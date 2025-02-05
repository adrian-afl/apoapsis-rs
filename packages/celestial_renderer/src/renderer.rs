use crate::atmosphere::atmosphere_drawer::AtmosphereDrawer;
use crate::atmosphere::clouds_generator_high_freq::CloudGeneratorHighFreq;
use crate::atmosphere::clouds_generator_low_freq::CloudGeneratorLowFreq;
use crate::buffers::common_buffer::CommonBuffer;
use crate::finalization::multi_merger::MultiMerger;
use crate::finalization::output::Output;
use crate::geometry::g_buffer::GBuffer;
use crate::geometry::icosphere_drawer::IcosphereDrawer;
use crate::geometry::mesh_drawer::MeshDrawer;
use crate::scene::celestial_hierarchy::CelestialHierarchy;
use crate::scene::material::Material;
use crate::scene::mesh::Mesh;
use glam::DVec4;
use renderer_common::camera::Camera;
use renderer_common::empty_textures::EMPTY_TEXTURES;
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tracing::{event, Level};
use ui_renderer::ui_drawer::UIDrawer;
use universe_simulation::body_definitions::BodyCelestialBodyDefinition;
use universe_simulation::simulation::Simulation;
use vengine_rs::core::semaphore::VESemaphore;
use vengine_rs::core::toolkit::VEToolkit;
use vengine_rs::graphics::vertex_buffer::VEVertexBuffer;

pub struct Renderer {
    config: ResolutionConfig,

    toolkit: Arc<VEToolkit>,

    pub g_buffer: GBuffer,

    pub common_buffer: CommonBuffer,

    ui_drawer: Arc<Mutex<UIDrawer>>,
    mesh_drawer: MeshDrawer,
    icosphere_drawer: IcosphereDrawer,
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
    ui_drawing_semaphore: Arc<Mutex<VESemaphore>>,
    outputting_semaphore: Arc<Mutex<VESemaphore>>,
    terrain_drawing_semaphore: Arc<Mutex<VESemaphore>>,
    water_drawing_semaphore: Arc<Mutex<VESemaphore>>,
}

impl Renderer {
    pub fn new(
        toolkit: Arc<VEToolkit>,
        config: &ResolutionConfig,
        ui_drawer: Arc<Mutex<UIDrawer>>,
    ) -> Self {
        EMPTY_TEXTURES.generate(&toolkit);

        let common_buffer = CommonBuffer::new(&toolkit).expect("Failed to create CommonBuffer");

        let mut g_buffer = GBuffer::new(&config, &toolkit).expect("Failed to create G-Buffer");
        let mesh_drawer = MeshDrawer::new(&config, &toolkit, &mut g_buffer, &common_buffer)
            .expect("Failed to create MeshDrawer");

        let icosphere_drawer =
            IcosphereDrawer::new(&toolkit, &config, &mut g_buffer, &common_buffer)
                .expect("Failed to create TerrainIcosphereDrawer");

        let mut multi_merger =
            MultiMerger::new(&config, &toolkit).expect("Failed to create MultiMerger");

        let output = Output::new(
            &config,
            &mut multi_merger,
            &mut ui_drawer.lock().unwrap(),
            &toolkit,
        )
        .expect("Failed to create Output");

        let mut cloud_generator_low_freq =
            CloudGeneratorLowFreq::new(&toolkit).expect("Failed to create CloudGeneratorLowFreq");
        let mut cloud_generator_high_freq =
            CloudGeneratorHighFreq::new(&toolkit).expect("Failed to create CloudGeneratorHighFreq");

        let mut atmosphere_drawer = AtmosphereDrawer::new(
            &config,
            &toolkit,
            &common_buffer,
            &mut cloud_generator_low_freq.low_freq_data_r,
            &mut cloud_generator_high_freq.high_freq_data_r,
            &mut g_buffer,
        )
        .expect("Failed to create AtmosphereDrawer");

        event!(Level::INFO, "multi_merger/update_inputs");
        multi_merger
            .update_inputs(
                &mut atmosphere_drawer.out_additive_rgb,
                &mut atmosphere_drawer.out_alpha_rgba,
                &config,
            )
            .expect("Failed to update multi_merger inputs");

        Self {
            config: config.clone(),
            toolkit: toolkit.clone(),
            ui_drawer,
            output,
            atmosphere_drawer,
            multi_merger,
            cloud_generator_high_freq,
            cloud_generator_low_freq,
            mesh_drawer,
            icosphere_drawer,
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
            ui_drawing_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
            outputting_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
            terrain_drawing_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
            water_drawing_semaphore: Arc::new(Mutex::from(toolkit.create_semaphore().unwrap())),
        }
    }

    pub fn draw(
        &mut self,
        meshes: &[&Mesh],
        universe_simulation: &Simulation,
        celestial_hierarchy: &mut CelestialHierarchy,
        camera: &Camera,
        total_time: f64,
        delta_time: f64,
    ) -> Result<(), RenderingError> {
        event!(Level::INFO, "Renderer draw START");
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        event!(Level::INFO, "common_buffer/update");
        self.common_buffer
            .update(camera, total_time)
            .expect("Failed to update common_buffer");

        event!(Level::INFO, "celestial_hierarchy/update");
        celestial_hierarchy.update(
            universe_simulation,
            &camera.position,
            &mut self.icosphere_drawer,
        )?;

        event!(Level::INFO, "celestial_hierarchy/get_rendered_bodies");
        let celestial_bodies = celestial_hierarchy.get_rendered_bodies();

        event!(Level::INFO, "mesh_drawer/record");
        self.mesh_drawer
            .record(meshes)
            .expect("Failed to record mesh_drawer");

        let mut swapchain = self.toolkit.swapchain.lock().unwrap();

        {
            let queue = &self
                .toolkit
                .queue
                .lock()
                .map_err(|_| RenderingError::QueueLockingFailed)?;
            event!(Level::INFO, "mesh_drawer/submit");
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

        let mut wait_for_semaphores = vec![self.mesh_drawing_semaphore.clone()];

        for mut body in celestial_bodies {
            event!(
                Level::INFO,
                "celestial_hierarchy/loop with {}",
                &body.body.name
            );

            event!(Level::INFO, "atmosphere_drawer/set_celestial_buffer");
            self.atmosphere_drawer
                .set_celestial_buffer(&body.celestial_body_buffer, &self.config)
                .expect("Failed to set_celestial_buffer for atmosphere_drawer");

            match &body.body.atmosphere {
                None => (),
                Some(atmosphere) => {
                    match &atmosphere.clouds {
                        None => (),
                        Some(_) => {
                            event!(Level::INFO, "cloud_generator_high_freq/update_buffer");
                            self.cloud_generator_high_freq
                                .update_buffer(
                                    DVec4::new(
                                        atmosphere.seed,
                                        atmosphere.seed, // TODO...
                                        atmosphere.seed,
                                        atmosphere.seed,
                                    ),
                                    total_time,
                                    1.0,
                                )
                                .expect("Failed to update cloud_generator_high_freq");
                            event!(Level::INFO, "cloud_generator_low_freq/update_buffer");
                            self.cloud_generator_low_freq
                                .update_buffer(
                                    DVec4::new(
                                        atmosphere.seed,
                                        atmosphere.seed,
                                        atmosphere.seed,
                                        atmosphere.seed,
                                    ),
                                    total_time,
                                    1.0,
                                )
                                .expect("Failed to update cloud_generator_low_freq");

                            let queue = &self
                                .toolkit
                                .queue
                                .lock()
                                .map_err(|_| RenderingError::QueueLockingFailed)?;
                            event!(Level::INFO, "cloud_generator_high_freq/submit");
                            self.cloud_generator_high_freq
                                .compute_stage
                                .command_buffer
                                .submit(
                                    queue,
                                    vec![],
                                    vec![self.clouds_generation_high_freq_semaphore.clone()],
                                )
                                .expect("Failed to compute cloud_generator_high_freq");

                            event!(Level::INFO, "cloud_generator_low_freq/submit");
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
                }
            }

            match &mut body.icosphere {
                None => (),
                Some(ref mut icosphere) => {
                    event!(Level::INFO, "icosphere_drawer/preload");
                    icosphere.preload(&self.toolkit)?;

                    event!(Level::INFO, "icosphere_drawer/record");
                    self.icosphere_drawer.record(icosphere)?;
                    let queue = &self
                        .toolkit
                        .queue
                        .lock()
                        .map_err(|_| RenderingError::QueueLockingFailed)?;

                    event!(Level::INFO, "terrain_icosphere_drawer/terrain submit");
                    self.icosphere_drawer
                        .terrain_render_stage
                        .command_buffer
                        .submit(
                            queue,
                            wait_for_semaphores.clone(),
                            vec![self.terrain_drawing_semaphore.clone()],
                        )
                        .expect("Failed to draw terain");
                    wait_for_semaphores = vec![self.terrain_drawing_semaphore.clone()];

                    event!(Level::INFO, "terrain_icosphere_drawer/water submit");
                    self.icosphere_drawer
                        .water_render_stage
                        .command_buffer
                        .submit(
                            queue,
                            wait_for_semaphores.clone(),
                            vec![self.water_drawing_semaphore.clone()],
                        )
                        .expect("Failed to draw water");
                    wait_for_semaphores = vec![self.water_drawing_semaphore.clone()];
                }
            }

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
                    event!(Level::INFO, "atmosphere_drawer/submit");
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

            let queue = &self
                .toolkit
                .queue
                .lock()
                .map_err(|_| RenderingError::QueueLockingFailed)?;

            event!(Level::INFO, "multi_merger/submit");
            self.multi_merger
                .compute_stage
                .command_buffer
                .submit(
                    queue,
                    wait_for_semaphores.clone(),
                    vec![self.multi_merging_semaphore.clone()],
                )
                .expect("Failed to compute multi_merger");
            wait_for_semaphores = vec![self.multi_merging_semaphore.clone()];

            queue.wait_idle().unwrap(); // TODO this could be better if i had a fence/ wait for semaphore before rerecording
                                        // but i could do it differently, if i had just 1 command buffer and record it and then just submit
                                        // todo later
        }

        {
            let queue = &self
                .toolkit
                .queue
                .lock()
                .map_err(|_| RenderingError::QueueLockingFailed)?;
            self.ui_drawer
                .lock()
                .unwrap()
                .render_stage
                .command_buffer
                .submit(
                    queue,
                    wait_for_semaphores.clone(),
                    vec![self.ui_drawing_semaphore.clone()],
                )
                .expect("Failed to draw ui");
            wait_for_semaphores = vec![self.ui_drawing_semaphore.clone()];
        }

        self.output
            .update_buffer(1.0)
            .expect("Failed to update output buffer");
        {
            let queue = &self
                .toolkit
                .queue
                .lock()
                .map_err(|_| RenderingError::QueueLockingFailed)?;
            event!(Level::INFO, "output/submit");
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

        event!(Level::INFO, "swapchain/blit");
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
        Ok(simulation.add_hierarchy(&self.config, body, None)?)
    }
}
