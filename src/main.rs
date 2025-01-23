use crate::app::GameWindowApp;
use std::fs::File;
use std::sync::{Arc, Mutex};
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::dpi::PhysicalSize;
use winit::window::WindowAttributes;

mod app;
mod body;
mod celestial_rendering;
mod config;
mod core;
mod ecs;
mod ecs_components;
mod ecs_systems;
mod input;
mod math;
mod simulation;
mod util;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_ansi(false)
        .with_writer(File::create("./log.txt").unwrap())
        .with_span_events(FmtSpan::FULL)
        .with_max_level(Level::WARN)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let window_attributes = WindowAttributes::default()
        .with_inner_size(PhysicalSize::new(640 * 3, 480 * 3))
        .with_title("dingus_mesh");

    VEToolkit::start(
        Box::from(|toolkit: Arc<VEToolkit>| {
            let app = GameWindowApp::new(toolkit);
            Arc::new(Mutex::from(app)) as Arc<Mutex<dyn App>>
        }),
        window_attributes,
    )
    .unwrap()
}
