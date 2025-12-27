mod api;
pub mod remote_controlled_game_stage;

use async_nats::client::traits::Publisher;
use async_nats::message::OutboundMessage;
use async_nats::{ConnectOptions, HeaderMap};
use futures_util::{FutureExt, StreamExt};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct IncomingRemoteIOMessage {
    pub name: String,
    pub payload: String,
    pub reply_to: Option<String>,
}

pub struct OutgoingRemoteIOMessage {
    pub name: String,
    pub payload: String,
    pub success: bool,
}

pub fn connect_nats(
    outbox: Arc<Mutex<VecDeque<OutgoingRemoteIOMessage>>>,
    inbox: Arc<Mutex<VecDeque<IncomingRemoteIOMessage>>>,
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
        .block_on(async_nats::connect_with_options(
            "nats://localhost:4222",
            ConnectOptions::new().no_echo(),
        ))
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
                    let mut headers = HeaderMap::new();
                    headers.insert("status", if message.success { "ok" } else { "error" });
                    rt.block_on(client.publish_message(OutboundMessage {
                        subject: message.name.into(),
                        reply: None, // server doesn't expect responses from the client
                        payload: message.payload.into(),
                        headers: Some(headers),
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
                    inbox.push_front(IncomingRemoteIOMessage {
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
