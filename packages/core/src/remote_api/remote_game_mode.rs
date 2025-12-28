use crate::remote_api::api::create_entity::create_entity;
use crate::remote_api::api::deserialize_world::deserialize_world;
// use crate::remote_api::api::generated::handle_message_components_api;
use crate::remote_api::api::generated::handle_message_components_api;
use crate::remote_api::api::reset_world::reset_world;
use crate::remote_api::api::serialize_world::serialize_world;
use ecs::ecs_world::{ECSWorld, ECSWorldSerializedRepresentation};
use nats::{NATS_CONNECTION, OutgoingRemoteIOMessage, send_event};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::string::ToString;
use ts_rs::TS;

pub struct RemoteGameMode {
    pub ecs: ECSWorld,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
pub struct OnRemoteGameModeInitializedEvent {
    pub ecs: ECSWorldSerializedRepresentation,
}

// @api_event on_remote_game_mode_initialized(null)

impl RemoteGameMode {
    pub fn new() -> Self {
        let ecs = ECSWorld::new();

        send_event!(
            "on_remote_game_mode_initialized",
            OnRemoteGameModeInitializedEvent {
                ecs: ecs.serialize()
            }
        );

        Self { ecs }
    }
}

fn handle_message(name: &str, payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    match name {
        "command.reset_world" => reset_world(payload, ecs),
        "command.serialize_world" => serialize_world(payload, ecs),
        "command.deserialize_world" => deserialize_world(payload, ecs),
        "command.create_entity" => create_entity(payload, ecs),
        _ => handle_message_components_api(name, payload, ecs),
    }
}

impl RemoteGameMode {
    pub fn update(&mut self) {
        let mut inbox = NATS_CONNECTION.inbox.lock().unwrap();
        while let Some(message) = inbox.pop_back() {
            println!("Message processing {}, {}", message.name, message.payload);

            let res = handle_message(&message.name, &message.payload, &mut self.ecs);

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
