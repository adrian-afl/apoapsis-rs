// use crate::remote_api::api::generated::handle_message_components_api;
use crate::remote_api::api::generated::handle_message_api;
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

// @api_event on_remote_game_mode_initialized()

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

impl RemoteGameMode {
    pub fn update(&mut self) {
        let mut inbox = NATS_CONNECTION.inbox.lock().unwrap();
        while let Some(message) = inbox.pop_back() {
            println!("Message processing {}, {}", message.name, message.payload);

            let res = handle_message_api(&message.name, &message.payload, &mut self.ecs);

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
