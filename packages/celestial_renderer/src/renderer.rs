use crate::atmosphere::atmosphere_drawer::AtmosphereDrawer;
use crate::atmosphere::clouds_generator_high_freq::CloudGeneratorHighFreq;
use crate::atmosphere::clouds_generator_low_freq::CloudGeneratorLowFreq;
use crate::buffers::common_buffer::CommonBuffer;
use crate::finalization::multi_merger::MultiMerger;
use crate::finalization::output::Output;
use crate::geometry::common_icosphere::PreloadResult;
use crate::geometry::g_buffer::GBuffer;
use crate::geometry::icosphere_drawer::IcosphereDrawer;
use crate::geometry::mesh_drawer::MeshDrawer;
use crate::scene::celestial_hierarchy::CelestialHierarchy;
use crate::scene::material::Material;
use crate::scene::mesh::Mesh;
use ash::vk;
use ash::vk::{AccessFlags, ImageAspectFlags, ImageLayout, PipelineStageFlags};
use common_util::{profile, udebug};
use glam::DVec4;
use math::decimal_vector_3d::DecimalVector3d;
use renderer_common::camera::Camera;
use renderer_common::empty_textures::EMPTY_TEXTURES;
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use ui_renderer::ui_drawer::UIDrawer;
use ui_renderer::ui_rendered_item::UIRenderedItem;
use ui_renderer::ui_system::UISystem;
use universe_simulation::body_definitions::BodyCelestialBodyDefinition;
use universe_simulation::simulation::Simulation;
use vengine_rs::core::command_buffer::VECommandBuffer;
use vengine_rs::core::memory_barrier::{submit_barriers, VEImageMemoryBarrier};
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

    command_buffer: VECommandBuffer,
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

        let mut multi_merger = MultiMerger::new(
            &config,
            &toolkit,
            &mut atmosphere_drawer.out_additive_rgb,
            &mut atmosphere_drawer.out_alpha_rgba,
        )
        .expect("Failed to create MultiMerger");

        let output = Output::new(
            &config,
            &mut multi_merger,
            &mut ui_drawer.lock().unwrap(),
            &toolkit,
        )
        .expect("Failed to create Output");

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

            command_buffer: toolkit.create_command_buffer().unwrap(),
        }
    }

    pub fn recreate_stages(&mut self) -> Result<(), RenderingError> {
        self.mesh_drawer.recreate_stage(&self.toolkit)?;
        self.icosphere_drawer.recreate_stage(&self.toolkit)?;
        self.cloud_generator_high_freq
            .recreate_stage(&self.toolkit)?;
        self.cloud_generator_low_freq
            .recreate_stage(&self.toolkit)?;
        self.atmosphere_drawer.recreate_stage(&self.toolkit)?;
        self.multi_merger.recreate_stage(&self.toolkit)?;
        // self.multi_merger // I might need to readd this
        //     .update_inputs(
        //         &self.toolkit.device,
        //         &mut self.atmosphere_drawer.out_additive_rgb,
        //         &mut self.atmosphere_drawer.out_alpha_rgba,
        //         &self.config,
        //     )
        //     .expect("Failed to update multi_merger inputs");

        self.output
            .recreate_stage(&self.toolkit, &mut self.multi_merger)?;

        Ok(())
    }

    pub fn record(
        &self,
        meshes: &[&Mesh],
        ui_items: &[&UIRenderedItem],
        celestial_hierarchy: &CelestialHierarchy,
    ) {
        self.command_buffer.begin().unwrap();

        self.common_buffer
            .record_copy_from_staging(&self.command_buffer);

        for mesh in meshes {
            mesh.mesh_buffer
                .lock()
                .unwrap()
                .record_copy_from_staging(&self.command_buffer);
        }

        self.output
            .buffer
            .record_copy_from_staging(&self.command_buffer);
        self.cloud_generator_low_freq
            .buffer
            .record_copy_from_staging(&self.command_buffer);
        self.cloud_generator_high_freq
            .buffer
            .record_copy_from_staging(&self.command_buffer);

        let celestial_bodies = celestial_hierarchy.get_rendered_bodies();

        self.mesh_drawer
            .record(&self.mesh_drawer.render_stage, &self.command_buffer, meshes);

        for (i, body) in celestial_bodies.iter().enumerate() {
            body.celestial_body_buffer
                .record_copy_from_staging(&self.command_buffer);

            unsafe {
                self.toolkit.device.device.cmd_clear_color_image(
                    self.command_buffer.handle,
                    self.g_buffer.color_rgb_roughness_a.handle,
                    self.g_buffer.color_rgb_roughness_a.current_layout,
                    &vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                    &[vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1) // TODO mip mapping
                        .base_array_layer(0)
                        .layer_count(1)],
                );

                self.toolkit.device.device.cmd_clear_color_image(
                    self.command_buffer.handle,
                    self.g_buffer.emission_rgb_metalness_a.handle,
                    self.g_buffer.emission_rgb_metalness_a.current_layout,
                    &vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                    &[vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1) // TODO mip mapping
                        .base_array_layer(0)
                        .layer_count(1)],
                );

                self.toolkit.device.device.cmd_clear_color_image(
                    self.command_buffer.handle,
                    self.g_buffer.normal_rgb_distance_a.handle,
                    self.g_buffer.normal_rgb_distance_a.current_layout,
                    &vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                    &[vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1) // TODO mip mapping
                        .base_array_layer(0)
                        .layer_count(1)],
                );

                self.toolkit.device.device.cmd_clear_depth_stencil_image(
                    self.command_buffer.handle,
                    self.g_buffer.shared_depth_buffer.handle,
                    self.g_buffer.shared_depth_buffer.current_layout,
                    &vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                    &[vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::DEPTH)
                        .base_mip_level(0)
                        .level_count(1) // TODO mip mapping
                        .base_array_layer(0)
                        .layer_count(1)],
                );
            }

            let barrier_color_rgb_roughness_a = VEImageMemoryBarrier {
                image: self.g_buffer.color_rgb_roughness_a.handle,
                aspect: ImageAspectFlags::COLOR,
                src_access: AccessFlags::SHADER_WRITE
                    | AccessFlags::MEMORY_WRITE
                    | AccessFlags::TRANSFER_WRITE,
                dst_access: AccessFlags::COLOR_ATTACHMENT_WRITE
                    | AccessFlags::COLOR_ATTACHMENT_READ,
                old_layout: ImageLayout::GENERAL,
                new_layout: ImageLayout::GENERAL,
            };

            let barrier_emission_rgb_metalness_a = VEImageMemoryBarrier {
                image: self.g_buffer.emission_rgb_metalness_a.handle,
                aspect: ImageAspectFlags::COLOR,
                src_access: AccessFlags::SHADER_WRITE
                    | AccessFlags::MEMORY_WRITE
                    | AccessFlags::TRANSFER_WRITE,
                dst_access: AccessFlags::COLOR_ATTACHMENT_WRITE
                    | AccessFlags::COLOR_ATTACHMENT_READ,
                old_layout: ImageLayout::GENERAL,
                new_layout: ImageLayout::GENERAL,
            };

            let barrier_normal_rgb_distance_a = VEImageMemoryBarrier {
                image: self.g_buffer.normal_rgb_distance_a.handle,
                aspect: ImageAspectFlags::COLOR,
                src_access: AccessFlags::SHADER_WRITE
                    | AccessFlags::MEMORY_WRITE
                    | AccessFlags::TRANSFER_WRITE,
                dst_access: AccessFlags::COLOR_ATTACHMENT_WRITE
                    | AccessFlags::COLOR_ATTACHMENT_READ,
                old_layout: ImageLayout::GENERAL,
                new_layout: ImageLayout::GENERAL,
            };

            submit_barriers(
                &self.toolkit.device,
                &self.command_buffer,
                PipelineStageFlags::ALL_COMMANDS,
                PipelineStageFlags::ALL_GRAPHICS,
                &[],
                &[],
                &[
                    barrier_color_rgb_roughness_a.build(),
                    barrier_emission_rgb_metalness_a.build(),
                    barrier_normal_rgb_distance_a.build(),
                ],
            );

            // let is_closest = i == 0;
            // if !is_closest {
            //     self.mesh_drawer.record(
            //         &self.mesh_drawer.render_stage,
            //         &self.command_buffer,
            //         meshes,
            //     );
            // } else {
            //     // todo temporary just to clear
            //     self.mesh_drawer
            //         .record(&self.mesh_drawer.render_stage, &self.command_buffer, &[]);
            // }

            self.cloud_generator_low_freq.record(&self.command_buffer);
            self.cloud_generator_high_freq.record(&self.command_buffer);
            match &body.terrain_icosphere {
                None => (),
                Some(icosphere) => {
                    self.icosphere_drawer
                        .record_terrain(&self.command_buffer, icosphere);
                }
            }

            match &body.water_icosphere {
                None => (),
                Some(icosphere) => {
                    self.icosphere_drawer
                        .record_water(&self.command_buffer, icosphere);
                }
            }

            let barrier_color_rgb_roughness_a = VEImageMemoryBarrier {
                image: self.g_buffer.color_rgb_roughness_a.handle,
                aspect: ImageAspectFlags::COLOR,
                src_access: AccessFlags::COLOR_ATTACHMENT_WRITE,
                dst_access: AccessFlags::SHADER_READ,
                old_layout: ImageLayout::GENERAL,
                new_layout: ImageLayout::GENERAL,
            };

            let barrier_emission_rgb_metalness_a = VEImageMemoryBarrier {
                image: self.g_buffer.emission_rgb_metalness_a.handle,
                aspect: ImageAspectFlags::COLOR,
                src_access: AccessFlags::COLOR_ATTACHMENT_WRITE,
                dst_access: AccessFlags::SHADER_READ,
                old_layout: ImageLayout::GENERAL,
                new_layout: ImageLayout::GENERAL,
            };

            let barrier_normal_rgb_distance_a = VEImageMemoryBarrier {
                image: self.g_buffer.normal_rgb_distance_a.handle,
                aspect: ImageAspectFlags::COLOR,
                src_access: AccessFlags::COLOR_ATTACHMENT_WRITE,
                dst_access: AccessFlags::SHADER_READ,
                old_layout: ImageLayout::GENERAL,
                new_layout: ImageLayout::GENERAL,
            };

            submit_barriers(
                &self.toolkit.device,
                &self.command_buffer,
                PipelineStageFlags::ALL_GRAPHICS,
                PipelineStageFlags::COMPUTE_SHADER,
                &[],
                &[],
                &[
                    barrier_color_rgb_roughness_a.build(),
                    barrier_emission_rgb_metalness_a.build(),
                    barrier_normal_rgb_distance_a.build(),
                ],
            );

            self.atmosphere_drawer
                .record(&self.command_buffer, &body.body_data_set, &self.config);

            self.multi_merger
                .record(&self.toolkit.device, &self.command_buffer, &self.config);
        }

        self.ui_drawer
            .lock()
            .unwrap()
            .record(&self.command_buffer, ui_items);

        self.output.record(&self.command_buffer, &self.multi_merger);
        self.command_buffer.end().unwrap();
    }

    pub fn draw(
        &mut self,
        meshes: &[&Mesh],
        ui_items: &[&UIRenderedItem],
        universe_simulation: &Simulation,
        celestial_hierarchy: &mut CelestialHierarchy,
        camera: &Camera,
        total_time: f64,
        delta_time: f64,
    ) -> Result<(), RenderingError> {
        profile!("common_buffer update", {
            self.common_buffer
                .update(camera, total_time)
                .expect("Failed to update common_buffer");
        });

        profile!("celestial_hierarchy update", {
            celestial_hierarchy.update(
                universe_simulation,
                &camera.position,
                &mut self.icosphere_drawer,
                &mut self.atmosphere_drawer,
            )?;
        });

        let mut celestial_bodies = profile!("get_rendered_bodies", {
            celestial_hierarchy.get_rendered_bodies_mut()
        });

        self.output.update_buffer(1.0)?;

        let mut swapchain = self.toolkit.swapchain.lock().unwrap();

        let mut any_updates = false;

        for (i, body) in celestial_bodies.iter_mut().enumerate() {
            let is_closest = i == 0;

            profile!("terrain", {
                match &mut body.terrain_icosphere {
                    None => (),
                    Some(ref mut icosphere) => {
                        let preload_result = if is_closest {
                            icosphere.preload(&self.toolkit)?
                        } else {
                            icosphere.preload(&self.toolkit)?
                        };
                        match preload_result {
                            PreloadResult::ChangesMade => {
                                any_updates = true;
                            }
                            PreloadResult::NotChanged => (),
                        }
                    }
                }
            });

            profile!("water", {
                match &mut body.water_icosphere {
                    None => (),
                    Some(ref mut icosphere) => {
                        let preload_result = if is_closest {
                            icosphere.preload(&self.toolkit)?
                        } else {
                            icosphere.preload(&self.toolkit)?
                        };

                        match preload_result {
                            PreloadResult::ChangesMade => {
                                any_updates = true;
                            }
                            PreloadResult::NotChanged => (),
                        }
                    }
                }
            });
        }

        if any_updates {
            // should aso check for meshes etc, maybe a trnasient entity to detect this
            self.record(meshes, ui_items, celestial_hierarchy);
        }
        {
            let queue = &self
                .toolkit
                .queue
                .lock()
                .map_err(|_| RenderingError::QueueLockingFailed)?;
            queue.wait_idle().unwrap();

            self.command_buffer
                .submit(
                    queue,
                    vec![swapchain.blit_done_semaphore.clone()],
                    vec![self.outputting_semaphore.clone()],
                )
                .expect("Failed to compute output");
        }
        profile!("blit to swapchain", {
            swapchain
                .blit(&self.output.output, vec![self.outputting_semaphore.clone()])
                .expect("Failed to blit to swapchain");
        });

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
