use crate::body::body_definitions::load_body_data;
use glam::DVec3;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use winit::event::MouseButton;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
enum ControlMapItem {
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

pub struct ControlQueue {
    events: Vec<ControlEvent>,
    control_map: ControlMap,
}

impl ControlQueue {
    pub fn new() -> Self {
        let input_json =
            fs::read_to_string("controls.json").expect("Failed to to read the controls.json file");
        let control_map: ControlMap = serde_json::from_str(&input_json).unwrap();
        Self {
            events: vec![],
            control_map,
        }
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: bool) {
        match button {
            MouseButton::Left => {}
            MouseButton::Right => {}
            MouseButton::Middle => {}
            MouseButton::Back => {}
            MouseButton::Forward => {}
            MouseButton::Other(_) => {}
        }
    }

    pub fn on_key(&mut self, key: PhysicalKey, state: bool) {
        match key {
            PhysicalKey::Code(key) => {
                if key == *self.control_map.keys.get(&ControlMapItem::Pause).unwrap() && state {
                    self.events.push(ControlEvent::Pause)
                }
            }
            PhysicalKey::Unidentified(_) => (),
        }
    }

    pub fn get_events(&self) -> Vec<ControlEvent> {
        self.events.clone()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
