use crate::app::GameWindowApp;
use crate::global_config::GLOBAL_CONFIG;
use core::game::Game;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::fmt::format::FmtSpan;
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowAttributes};

mod app;
mod global_config;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_ansi(false)
        .with_writer(File::create("./log.txt").unwrap())
        .with_span_events(FmtSpan::FULL)
        .with_max_level(GLOBAL_CONFIG.log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    if GLOBAL_CONFIG.headless {
        let mut game = Game::new_headless();
        println!("Headless loop starting...");
        loop {
            game.update();
            thread::sleep(Duration::from_millis(10));
        }
    } else {
        let window_attributes = WindowAttributes::default()
            .with_inner_size(PhysicalSize::new(640 * 2, 480 * 2))
            .with_title("Codename T.S.P.");

        VEToolkit::start(
            Box::from(move |toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>| {
                let app = GameWindowApp::new(toolkit, window);
                Arc::new(Mutex::from(app)) as Arc<Mutex<dyn App>>
            }),
            window_attributes,
        )
        .unwrap()
    }
}
