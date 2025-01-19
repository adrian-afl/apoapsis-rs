use crate::body::body_definitions::{load_body_data, BodyTerrain};
use crate::celestial_rendering::atmosphere::atmosphere_drawer::AtmosphereDrawer;
use crate::celestial_rendering::atmosphere::clouds_generator_high_freq::CloudGeneratorHighFreq;
use crate::celestial_rendering::atmosphere::clouds_generator_low_freq::CloudGeneratorLowFreq;
use crate::celestial_rendering::buffers::celestial_body_buffer::CelestialBodyBuffer;
use crate::celestial_rendering::buffers::common_buffer::CommonBuffer;
use crate::celestial_rendering::finalization::multi_merger::MultiMerger;
use crate::celestial_rendering::finalization::output::Output;
use crate::celestial_rendering::geometry::g_buffer::GBuffer;
use crate::celestial_rendering::geometry::mesh_drawer::MeshDrawer;
use crate::celestial_rendering::scene::camera::Camera;
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

pub struct CelestialRendererApp {
    start_time: f64,
    last_time: f64,

    pub config: Config,

    pub toolkit: Arc<VEToolkit>,

    meshes: Vec<Mesh>,
    camera: Camera,

    pub g_buffer: GBuffer,

    pub common_buffer: CommonBuffer,

    mesh_drawer: MeshDrawer,
    pub simulation: Simulation,
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

#[allow(clippy::unwrap_used)]
impl CelestialRendererApp {
    pub fn new(toolkit: Arc<VEToolkit>) -> CelestialRendererApp {
        EMPTY_TEXTURES.generate(&toolkit);

        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let config = Config::new(640, 480);

        let common_buffer = CommonBuffer::new(&toolkit).expect("Failed to create CommonBuffer");
        let celestial_body_buffer =
            CelestialBodyBuffer::new(&toolkit).expect("Failed to create CelestialBodyBuffer");

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

        let mut simulation = Simulation::new(toolkit.clone());

        let snowball = load_body_data("universe/snowball/body.json");

        simulation
            .add_hierarchy(&config, &mut g_buffer, &common_buffer, &snowball, None)
            .expect("Failed to add a body to the simulation");

        CelestialRendererApp {
            start_time,
            last_time: start_time,
            config,
            toolkit: toolkit.clone(),
            output,
            atmosphere_drawer,
            multi_merger,
            cloud_generator_high_freq,
            cloud_generator_low_freq,
            mesh_drawer,
            g_buffer,
            meshes: vec![],
            camera: Camera::new(),
            common_buffer,
            simulation,

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
}

#[allow(clippy::unwrap_used)]
impl App for CelestialRendererApp {
    fn draw(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let elapsed = now - self.start_time;
        let delta_time = now - self.last_time;
        self.last_time = now;

        self.camera
            .set_perspective(90.0 * (PI / 180.0), 640.0 / 480.0, 0.1, 63780000.0);

        self.camera.position = DecimalVector3d::from_str("0.0", "0.0", "0.0")
            - DecimalVector3d::from_str("0.0", "0.0", "12756000.0");

        self.camera.orientation = DQuat::from_mat4(&DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 0.0, 1.0),
            DVec3::new(0.0, 1.0, 0.0),
        )); // * DQuat::from_axis_angle(DVec3::new(0.0, 1.0, 0.0), elapsed);

        self.camera.update();

        self.common_buffer
            .update(&self.camera, elapsed)
            .expect("Failed to update common_buffer");

        event!(Level::WARN, "Recording mesh_drawer");
        self.mesh_drawer
            .record(self.meshes.as_slice())
            .expect("Failed to record mesh_drawer");

        let mut swapchain = self.toolkit.swapchain.lock().unwrap();

        event!(Level::WARN, "Submitting mesh_drawer");
        self.mesh_drawer
            .render_stage
            .command_buffer
            .submit(
                &self.toolkit.queue,
                vec![swapchain.blit_done_semaphore.clone()],
                vec![self.mesh_drawing_semaphore.clone()],
            )
            .expect("Failed to draw mesh_drawer");

        self.simulation
            .update(&self.camera.position, &f64_to_dbig(now * 6000.0)); // TODO use some game time!!

        let closest_hierarchy = self
            .simulation
            .find_closest_hierarchy(&self.camera.position);

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

                            self.cloud_generator_high_freq
                                .compute_stage
                                .command_buffer
                                .submit(
                                    &self.toolkit.queue,
                                    vec![],
                                    vec![self.clouds_generation_high_freq_semaphore.clone()],
                                )
                                .expect("Failed to compute cloud_generator_high_freq");

                            self.cloud_generator_low_freq
                                .compute_stage
                                .command_buffer
                                .submit(
                                    &self.toolkit.queue,
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
                    drawer
                        .lock()
                        .unwrap()
                        .render_stage
                        .command_buffer
                        .submit(
                            &self.toolkit.queue,
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
                    drawer
                        .lock()
                        .unwrap()
                        .render_stage
                        .command_buffer
                        .submit(
                            &self.toolkit.queue,
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

                    self.atmosphere_drawer
                        .compute_stage
                        .command_buffer
                        .submit(
                            &self.toolkit.queue,
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
            self.multi_merger
                .compute_stage
                .command_buffer
                .submit(
                    &self.toolkit.queue,
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
        self.output
            .compute_stage
            .command_buffer
            .submit(
                &self.toolkit.queue,
                wait_for_semaphores.clone(),
                vec![self.outputting_semaphore.clone()],
            )
            .expect("Failed to compute output");

        event!(Level::WARN, "Submitting blit");
        swapchain
            .blit(&self.output.output, vec![self.outputting_semaphore.clone()])
            .expect("Failed to blit to swapchain");
    }
}
