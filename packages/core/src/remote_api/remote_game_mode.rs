use crate::game_context::GameContext;
use crate::remote_api::api::create_entity::create_entity;
use crate::remote_api::api::deserialize_world::deserialize_world;
// use crate::remote_api::api::generated::handle_message_components_api;
use crate::remote_api::api::generated::handle_message_components_api;
use crate::remote_api::api::reset_world::reset_world;
use crate::remote_api::api::serialize_world::serialize_world;
use crate::remote_api::nats::{IncomingRemoteIOMessage, OutgoingRemoteIOMessage, connect_nats};
use ecs::ecs_world::ECSWorld;
use std::collections::VecDeque;
use std::string::ToString;
use std::sync::{Arc, Mutex};

pub struct RemoteGameMode {
    pub ecs: ECSWorld,
    outbox: Arc<Mutex<VecDeque<OutgoingRemoteIOMessage>>>,
    inbox: Arc<Mutex<VecDeque<IncomingRemoteIOMessage>>>,
}

impl RemoteGameMode {
    pub fn new() -> Self {
        let mut ecs = ECSWorld::new();
        let outbox: Arc<Mutex<VecDeque<OutgoingRemoteIOMessage>>> =
            Arc::new(Mutex::new(VecDeque::new()));
        let inbox: Arc<Mutex<VecDeque<IncomingRemoteIOMessage>>> =
            Arc::new(Mutex::new(VecDeque::new()));

        outbox.lock().unwrap().push_front(OutgoingRemoteIOMessage {
            name: "event.game-ready".to_string(),
            payload: "void".to_string(),
            success: true,
        });

        connect_nats(outbox.clone(), inbox.clone());

        Self { ecs, outbox, inbox }
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
        let mut inbox = self.inbox.lock().unwrap();
        while let Some(message) = inbox.pop_back() {
            println!("Message processing {}, {}", message.name, message.payload);

            let res = handle_message(&message.name, &message.payload, &mut self.ecs);

            match res {
                Ok(result) => self
                    .outbox
                    .lock()
                    .unwrap()
                    .push_front(OutgoingRemoteIOMessage {
                        name: message.reply_to.expect("No reply-to set"),
                        payload: result.unwrap_or("{}".to_string()),
                        success: true,
                    }),
                Err(error) => {
                    eprintln!("Error while processing a message: {}", error);
                    self.outbox
                        .lock()
                        .unwrap()
                        .push_front(OutgoingRemoteIOMessage {
                            name: message.reply_to.expect("No reply-to set"),
                            payload: error,
                            success: false,
                        })
                }
            }
        }
    }
}
