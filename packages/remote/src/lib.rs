mod api;
pub mod remote_controlled_game_stage;

use async_nats::client::traits::Publisher;
use async_nats::message::OutboundMessage;
use futures_util::{FutureExt, StreamExt};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

pub fn create_message_id() -> String {
    Uuid::new_v4().to_string()
}

pub struct RemoteIOMessage {
    pub name: String,
    pub payload: String,
    pub reply_to: Option<String>,
}

pub fn connect_nats(
    outbox: Arc<Mutex<VecDeque<RemoteIOMessage>>>,
    inbox: Arc<Mutex<VecDeque<RemoteIOMessage>>>,
) {
    println!("Connecting to NATS...");
    println!("Connecting from a new thread...");
    let rt = Arc::new(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap(),
    );

    let client = rt
        .block_on(async_nats::connect("nats://localhost:4222"))
        .unwrap();

    println!("Connected to NATS, subscribing to all...");

    let mut subscription = rt.block_on(client.subscribe("command.*")).unwrap();

    {
        let rt = rt.clone();
        thread::spawn(move || {
            println!("NATS transmit loop starting...");
            loop {
                let mut outbox = outbox.lock().unwrap();
                while !outbox.is_empty() {
                    let message = outbox.pop_back().unwrap();
                    println!("B");
                    rt.block_on(client.publish_message(OutboundMessage {
                        subject: message.name.into(),
                        reply: None, // server doesn't expect responses from the client
                        payload: message.payload.into(),
                        headers: None,
                    }))
                    .unwrap();
                }
            }
        });
    }
    {
        let rt = rt.clone();
        thread::spawn(move || {
            println!("NATS receive loop starting...");
            loop {
                if let Some(message) = rt.block_on(subscription.next()) {
                    let mut inbox = inbox.lock().unwrap();
                    println!("X {}", message.subject.as_str());
                    inbox.push_front(RemoteIOMessage {
                        name: message.subject.into_string(),
                        payload: String::from_utf8(Vec::from(message.payload))
                            .expect("utf8 parse failed"),
                        reply_to: message.reply.map(|x| x.to_string()),
                    })
                }
            }
        });
    }

    // You need to drop subscripions in async context, as they do spawn tasks to clean themselves up.
    // rt.block_on(async {
    //     drop(subscription);
    //     drop(client);
    // });
}
