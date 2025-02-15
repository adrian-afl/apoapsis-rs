use crate::app::GameWindowApp;
use crate::cli_args::CLIArgs;
use clap::Parser;
use common_util::udebug;
use common_util::udp_debugging::UDP_DEBUGGING;
use std::fs::File;
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowAttributes};

mod app;
mod cli_args;

fn main() {
    let cli_args = Arc::new(CLIArgs::parse());

    UDP_DEBUGGING.set_target("127.0.0.1:6000");

    udebug!("test without formatting");
    udebug!("test with formatting {} + {} = {}", 1, 2, 1 + 2);

    let subscriber = FmtSubscriber::builder()
        .with_ansi(false)
        .with_writer(File::create("./log.txt").unwrap())
        .with_span_events(FmtSpan::FULL)
        .with_max_level(cli_args.log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let window_attributes = WindowAttributes::default()
        .with_inner_size(PhysicalSize::new(640 * 3, 480 * 3))
        .with_title("Codename T.S.P.");

    VEToolkit::start(
        Box::from(move |toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>| {
            let app = GameWindowApp::new(toolkit, window, cli_args.clone());
            Arc::new(Mutex::from(app)) as Arc<Mutex<dyn App>>
        }),
        window_attributes,
    )
    .unwrap()
}
