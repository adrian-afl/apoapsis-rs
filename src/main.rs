use crate::app::GameWindowApp;
use config::GLOBAL_CONFIG;
use core::game::Game;
use nats::send_event;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use ts_rs::TS;
use vengine_rs::core::toolkit::{App, VEToolkit};
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowAttributes};

mod app;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct OnGameBootReadyEventData {
    pub headless: bool,
}

// @api_event on_game_boot_ready(OnGameBootReadyEventData)

fn main() {
    println!("{:?}", GLOBAL_CONFIG);

    if GLOBAL_CONFIG.headless {
        println!("Creating headless instance...");
        let mut game = Game::new_headless();
        println!("Headless loop starting...");

        send_event!(
            "on_game_boot_ready",
            OnGameBootReadyEventData { headless: true }
        );

        loop {
            game.update();
            thread::sleep(Duration::from_millis(10));
        }
    } else {
        println!("Window loop starting...");
        let window_attributes = WindowAttributes::default()
            .with_inner_size(PhysicalSize::new(640 * 2, 480 * 2))
            .with_title("Codename T.S.P.");

        thread::sleep(Duration::from_millis(500));

        VEToolkit::start(
            Box::from(move |toolkit: Arc<VEToolkit>, window: Arc<Mutex<Window>>| {
                thread::sleep(Duration::from_millis(500));
                let app = GameWindowApp::new(toolkit, window);
                thread::sleep(Duration::from_millis(500));
                Arc::new(Mutex::from(app)) as Arc<Mutex<dyn App>>
            }),
            window_attributes,
        )
        .unwrap();

        thread::sleep(Duration::from_millis(500));

        send_event!(
            "on_game_boot_ready",
            OnGameBootReadyEventData { headless: false }
        );
    }
}
