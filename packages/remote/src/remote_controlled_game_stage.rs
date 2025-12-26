use crate::api::create_entity::create_entity;
use crate::api::deserialize_world::deserialize_world;
use crate::api::generated::handle_message_components_api;
use crate::api::reset_world::reset_world;
use crate::api::serialize_world::serialize_world;
use crate::{IncomingRemoteIOMessage, OutgoingRemoteIOMessage, connect_nats};
use core::game_context::GameContext;
use core::game_stage_trait::GameStage;
use core::game_stage_trait::StageTransition;
use ecs::ecs_world::ECSWorld;
use ecs::entity::ENTITY_SEQ;
use std::collections::VecDeque;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

pub struct RemoteControlledGameStage {
    ecs: ECSWorld,
    outbox: Arc<Mutex<VecDeque<OutgoingRemoteIOMessage>>>,
    inbox: Arc<Mutex<VecDeque<IncomingRemoteIOMessage>>>,
}

impl RemoteControlledGameStage {
    pub fn new(context: &GameContext) -> Self {
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

fn handle_message(
    name: &str,
    payload: &str,
    ecs: &mut ECSWorld,
    context: &GameContext,
) -> Result<String, String> {
    match name {
        "command.reset_world" => Ok(reset_world(payload, ecs)),
        "command.serialize_world" => Ok(serialize_world(payload, ecs)),
        "command.deserialize_world" => Ok(deserialize_world(payload, ecs)),
        "command.create_entity" => Ok(create_entity(payload, ecs)),
        _ => handle_message_components_api(name, payload, ecs),
    }
}

impl GameStage for RemoteControlledGameStage {
    fn update(&mut self, context: &GameContext) -> StageTransition {
        {
            let mut inbox = self.inbox.lock().unwrap();
            while let Some(message) = inbox.pop_back() {
                println!("Message processing {}, {}", message.name, message.payload);

                let (name, id) = match message.name.rfind('.') {
                    Some(pos) => (&message.name[..pos], &message.name[pos + 1..]),
                    None => continue, // No dot found: everything is the "before" part
                };

                let res = handle_message(&name, &message.payload, &mut self.ecs, context);

                match res {
                    Ok(result) => self
                        .outbox
                        .lock()
                        .unwrap()
                        .push_front(OutgoingRemoteIOMessage {
                            name: message.reply_to.expect("No reply-to set"),
                            payload: result,
                            success: true,
                        }),
                    Err(error) => {
                        eprintln!("Error while processing a message: {error}");
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
        StageTransition::DoNothing
    }

    fn get_ecs_world(&mut self) -> &mut ECSWorld {
        &mut self.ecs
    }
}
