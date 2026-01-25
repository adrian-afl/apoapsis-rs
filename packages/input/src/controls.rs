use crate::controls_mapping::{ControlMapItem, ControlsMapping};
use crate::mouse_input::MouseInput;
use gilrs::{Event, EventType, Gamepad, GamepadId, Gilrs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ts_rs::TS;
use winit::event::MouseButton;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum ControlEvent {
    ControlActivate(ControlMapItem),
    ControlRelease(ControlMapItem),
    RawKeyDown(u32),
    RawKeyUp(u32),
    RawText(String),
}

pub struct Controls {
    mapping: ControlsMapping,
    new_events: Vec<ControlEvent>,
    pub mouse: MouseInput,
    controls_state: HashMap<ControlMapItem, bool>,
    gamepad_helper: Gilrs,
    active_gamepad: Option<GamepadId>,
}

impl Controls {
    pub fn new(window: Arc<Mutex<Window>>) -> Self {
        Self {
            new_events: Vec::new(),
            mouse: MouseInput::new(window.clone()),
            mapping: ControlsMapping::new(),
            controls_state: HashMap::new(),
            gamepad_helper: Gilrs::new().unwrap(),
            active_gamepad: None,
        }
    }

    pub fn get_control_state(&self, control: ControlMapItem) -> bool {
        *self.controls_state.get(&control).unwrap_or(&false)
    }

    pub fn was_control_activated(&self, control: ControlMapItem) -> bool {
        self.new_events
            .contains(&ControlEvent::ControlActivate(control))
    }

    pub fn was_control_released(&self, control: ControlMapItem) -> bool {
        self.new_events
            .contains(&ControlEvent::ControlRelease(control))
    }

    pub fn on_mouse_button(&mut self, button: MouseButton, state: bool) {
        let mapped = self.mapping.map_mouse_button_event(button, state);
        for event in mapped {
            self.handle_control_event(event);
        }
    }

    pub fn on_key(&mut self, key: PhysicalKey, state: bool, text: &str) {
        let mapped = self.mapping.map_keyboard_event(key, state, text);
        for event in mapped {
            self.handle_control_event(event);
        }
    }

    pub fn update_gamepad_helper(&mut self) {
        while let Some(Event {
            id, event, time, ..
        }) = self.gamepad_helper.next_event()
        {
            println!("{:?} New event from {}: {:?}", time, id, event);
            self.active_gamepad = Some(id);
            match event {
                EventType::ButtonPressed(b, _) => {
                    let mapped = self.mapping.map_gamepad_event(b, true);
                    for event in mapped {
                        self.handle_control_event(event);
                    }
                }
                EventType::ButtonReleased(b, _) => {
                    let mapped = self.mapping.map_gamepad_event(b, false);
                    for event in mapped {
                        self.handle_control_event(event);
                    }
                }
                EventType::ButtonRepeated(_, _) => {}
                EventType::ButtonChanged(_, _, _) => {}
                EventType::AxisChanged(_, _, _) => {}
                EventType::Connected => {}
                EventType::Disconnected => {}
                EventType::Dropped => {}
                EventType::ForceFeedbackEffectCompleted => {}
                _ => {}
            }
        }
    }

    pub fn get_active_gamepad(&self) -> Option<Gamepad<'_>> {
        self.active_gamepad
            .map(|id| self.gamepad_helper.gamepad(id))
    }

    fn handle_control_event(&mut self, event: ControlEvent) {
        let item_state = match &event {
            ControlEvent::ControlActivate(item) => Some((item, true)),
            ControlEvent::ControlRelease(item) => Some((item, false)),
            _ => None,
        };
        if let Some(item_state) = item_state {
            let (item, state) = item_state;
            match self.controls_state.get_mut(item) {
                None => {
                    self.controls_state.insert(item.clone(), state);
                }
                Some(current) => *current = state,
            }
        }
        self.new_events.push(event);
    }

    pub fn get_new_events(&self) -> &Vec<ControlEvent> {
        &self.new_events
    }

    pub fn clear_events(&mut self) {
        self.new_events = Vec::new();
    }
}
