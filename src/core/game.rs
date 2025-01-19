use std::sync::{Arc, Mutex};
use vengine_rs::core::semaphore::VESemaphore;
use vengine_rs::core::toolkit::VEToolkit;
use crate::app::CelestialRendererApp;
use crate::celestial_rendering::atmosphere::atmosphere_drawer::AtmosphereDrawer;
use crate::celestial_rendering::atmosphere::clouds_generator_high_freq::CloudGeneratorHighFreq;
use crate::celestial_rendering::atmosphere::clouds_generator_low_freq::CloudGeneratorLowFreq;
use crate::celestial_rendering::buffers::common_buffer::CommonBuffer;
use crate::celestial_rendering::finalization::multi_merger::MultiMerger;
use crate::celestial_rendering::finalization::output::Output;
use crate::celestial_rendering::geometry::g_buffer::GBuffer;
use crate::celestial_rendering::geometry::mesh_drawer::MeshDrawer;
use crate::celestial_rendering::scene::camera::Camera;
use crate::celestial_rendering::scene::mesh::Mesh;
use crate::config::Config;
use crate::simulation::simulation::Simulation;

pub struct Game{
    start_time: f64,
    last_time: f64,

    pub config: Config,
    
    camera: Camera,
    
    renderer: CelestialRendererApp;
}