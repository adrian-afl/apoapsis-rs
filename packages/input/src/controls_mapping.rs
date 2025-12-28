use crate::controls::ControlEvent;
use common_util::strip_json_line_comments::strip_json_line_comments;
use gilrs::Button;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use winit::event::MouseButton;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize)]
pub enum ControlMapItem {
    Pause,
    MenuClickPrimary,
    MenuClickSecondary,

    Start,
    Confirm,
    Cancel,

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

    FlightKillRotation,

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

    // DEBUG
    RecompileShaders,
    DevConsole,

    DebugIncreaseTranslationSpeed,
    DebugDecreaseTranslationSpeed,

    DebugZoomIn,
    DebugZoomOut,

    DebugMoreExposure,
    DebugLessExposure,

    DebugMouseLeft,
}

#[derive(Debug, Clone, Deserialize)]
// #[serde(rename_all = "camelCase")] // probably not a good idea
struct ControlMap {
    pub keys: HashMap<ControlMapItem, KeyCode>,
    pub mouse_buttons: HashMap<ControlMapItem, MouseButton>,
    pub gamepad_buttons: HashMap<ControlMapItem, Button>,
}

pub struct ControlsMapping {
    control_map: ControlMap,
}

impl Default for ControlsMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlsMapping {
    pub fn new() -> Self {
        Self {
            control_map: ControlsMapping::load("controls.json"),
        }
    }

    pub fn load(file: &str) -> ControlMap {
        let input_json =
            fs::read_to_string(file).unwrap_or_else(|_| panic!("Failed to to read the {file} file"));
        serde_json::from_str(&strip_json_line_comments(&input_json)).unwrap()
    }

    pub fn map_mouse_button_event(&self, button: MouseButton, state: bool) -> Vec<ControlEvent> {
        let mut result = vec![];
        for entry in self.control_map.mouse_buttons.iter() {
            let control_map_item = entry.0;
            let mouse_button = entry.1;
            if button == *mouse_button {
                result.push(match state {
                    true => ControlEvent::ControlActivate(control_map_item.clone()),
                    false => ControlEvent::ControlRelease(control_map_item.clone()),
                });
            }
        }
        result
    }

    pub fn map_keyboard_event(&self, key: PhysicalKey, state: bool) -> Vec<ControlEvent> {
        // println!("{:?}", key);
        let mut result = vec![];
        match key {
            PhysicalKey::Code(key) => {
                for entry in self.control_map.keys.iter() {
                    let control_map_item = entry.0;
                    let key_code = entry.1;
                    if key == *key_code {
                        result.push(match state {
                            true => ControlEvent::ControlActivate(control_map_item.clone()),
                            false => ControlEvent::ControlRelease(control_map_item.clone()),
                        });
                    }
                }
            }
            PhysicalKey::Unidentified(_) => (),
        }
        result
    }

    pub fn map_gamepad_event(&self, button: Button, state: bool) -> Vec<ControlEvent> {
        let mut result = vec![];
        for entry in self.control_map.gamepad_buttons.iter() {
            let control_map_item = entry.0;
            let gamepad_button = entry.1;
            if button == *gamepad_button {
                result.push(match state {
                    true => ControlEvent::ControlActivate(control_map_item.clone()),
                    false => ControlEvent::ControlRelease(control_map_item.clone()),
                });
            }
        }
        result
    }
}
