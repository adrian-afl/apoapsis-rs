use crate::app::GameWindowApp;
use config::GLOBAL_CONFIG;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tcpapi::send_event;
use ts_rs::TS;
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowAttributes};

mod app;

// @api_event on_game_boot_ready()
// @api_event startup()

fn main() {
    send_event!("startup");

    println!("Window loop starting...");
    let window_attributes = WindowAttributes::default()
        .with_inner_size(PhysicalSize::new(640 * 3, 480 * 3))
        .with_title("Codename T.S.P.");

    thread::sleep(Duration::from_millis(500));

    VEToolkit::start(
        Box::from(move |toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>| {
            thread::sleep(Duration::from_millis(500));
            let app = GameWindowApp::new(toolkit, window);
            thread::sleep(Duration::from_millis(500));

            send_event!("on_game_boot_ready");
            Arc::new(Mutex::from(app)) as Arc<Mutex<dyn App>>
        }),
        window_attributes,
    )
    .unwrap();
}
