use std::collections::VecDeque;
use std::ops::Deref;
// use crate::remote_api::api::generated::handle_message_components_api;
use crate::remote_api::api::generated::handle_message_api;
use celestial_renderer::rendering_system::RenderingSystem;
use ecs::ecs_world::{ECSWorld, ECSWorldSerializedRepresentation};
use nats::{IncomingRemoteIOMessage, NATS_CONNECTION, OutgoingRemoteIOMessage, send_event};
use real_physics_engine::physics_system::PhysicsSystem;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::string::ToString;
use ts_rs::TS;
use universe_simulation::simulation::Simulation;

pub struct RemoteGameExecutionContext<'a> {
    pub ecs: &'a mut ECSWorld,
    pub simulation: &'a mut Simulation,
    pub physics_system: &'a mut PhysicsSystem,
}

pub struct RemoteGameMode {
    pub ecs: ECSWorld,
}

impl RemoteGameMode {
    pub fn new() -> Self {
        let ecs = ECSWorld::new();

        // @api_event on_remote_game_mode_initialized()
        send_event!("on_remote_game_mode_initialized");

        Self { ecs }
    }
}

impl RemoteGameMode {
    pub fn update(&mut self, simulation: &mut Simulation, physics_system: &mut PhysicsSystem) {
        let mut inbox: VecDeque<IncomingRemoteIOMessage> = {
            let mut guard = NATS_CONNECTION.inbox.lock().unwrap();
            let clone = guard.clone();
            guard.clear();
            clone
        };
        while let Some(message) = inbox.pop_back() {
            // println!("Message processing {}, {}", message.name, message.payload);

            let res = {
                let mut full_context = RemoteGameExecutionContext {
                    ecs: &mut self.ecs,
                    simulation,
                    physics_system,
                };
                handle_message_api(&message.name, &message.payload, &mut full_context)
            };

            match res {
                Ok(result) => {
                    NATS_CONNECTION
                        .outbox
                        .lock()
                        .unwrap()
                        .push_front(OutgoingRemoteIOMessage {
                            name: message.reply_to.expect("No reply-to set"),
                            payload: result.unwrap_or("{}".to_string()),
                            success: true,
                        })
                }
                Err(error) => {
                    eprintln!("Error while processing a message: {}", error);
                    NATS_CONNECTION
                        .outbox
                        .lock()
                        .unwrap()
                        .push_front(OutgoingRemoteIOMessage {
                            name: message.reply_to.expect("No reply-to set"),
                            payload: json!({ "error": error }).to_string(),
                            success: false,
                        })
                }
            }
        }
    }
}
