use crate::body::body_definitions::load_body_data;
use crate::core::game_event_system::GameEvent::{ControlActivate, ControlRelease};
use crate::core::game_event_system::GameEventSystem;
use glam::DVec3;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use winit::event::MouseButton;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum ControlMapItem {
    Pause,
    MenuClickPrimary,
    MenuClickSecondary,

    WalkLeft,
    WalkRight,
    WalkForwards,
    WalkBackwards,
    Use,
    OnFootShoot,
    OnFootCrouch,
    OnFootZoom,

    FlightPitchAxis,
    FlightPitchUp,
    FlightPitchDown,

    FlightYawAxis,
    FlightYawLeft,
    FlightYawRight,

    FlightRollAxis,
    FlightRollLeft,
    FlightRollRight,

    FlightCameraModeSwitch,
    FlightCameraFrameSwitch,
    FlightZoom,

    FlightTranslateXAxis,
    FlightTranslateLeft,
    FlightTranslateRight,

    FlightTranslateYAxis,
    FlightTranslateUp,
    FlightTranslateDown,

    FlightTranslateZAxis,
    FlightTranslateForwards,
    FlightTranslateBackwards,

    FlightExit,
    FlightShoot,
}

#[derive(Debug, Clone, Deserialize)]
// #[serde(rename_all = "camelCase")] // probably not a good idea
struct ControlMap {
    pub keys: HashMap<ControlMapItem, KeyCode>,
    pub mouse_buttons: HashMap<ControlMapItem, MouseButton>,
}

#[derive(Debug, Clone)]
pub enum ControlEvent {
    Pause,
}

pub struct Controls {
    control_map: ControlMap,
    game_event_system: Arc<GameEventSystem>,
}

impl Controls {
    pub fn new(game_event_system: Arc<GameEventSystem>) -> Self {
        let input_json =
            fs::read_to_string("controls.json").expect("Failed to to read the controls.json file");
        let control_map: ControlMap = serde_json::from_str(&input_json).unwrap();
        Self {
            control_map,
            game_event_system,
        }
    }

    pub fn on_mouse_button(&self, button: MouseButton, state: bool) {
        for entry in self.control_map.mouse_buttons.iter() {
            let control_map_item = entry.0;
            let mouse_button = entry.1;
            if button == *mouse_button {
                match state {
                    true => self
                        .game_event_system
                        .push(ControlActivate(control_map_item.clone())),
                    false => self
                        .game_event_system
                        .push(ControlRelease(control_map_item.clone())),
                }
            }
        }
    }

    pub fn on_key(&self, key: PhysicalKey, state: bool) {
        match key {
            PhysicalKey::Code(key) => {
                for entry in self.control_map.keys.iter() {
                    let control_map_item = entry.0;
                    let key_code = entry.1;
                    if key == *key_code {
                        match state {
                            true => self
                                .game_event_system
                                .push(ControlActivate(control_map_item.clone())),
                            false => self
                                .game_event_system
                                .push(ControlRelease(control_map_item.clone())),
                        }
                    }
                }
            }
            PhysicalKey::Unidentified(_) => (),
        }
    }
}
