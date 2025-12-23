mod api;
pub mod remote_controlled_game_stage;

use futures_util::{FutureExt, StreamExt};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub static MESSAGE_SEQ: AtomicU64 = AtomicU64::new(1);

pub struct RemoteIOMessage {
    pub id: u64,
    pub name: String,
    pub payload: String,
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
                    rt.block_on(client.publish(message.name, message.payload.into()))
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
                        id: MESSAGE_SEQ.fetch_add(1, Ordering::SeqCst),
                        name: message.subject.into_string(),
                        payload: String::from_utf8(Vec::from(message.payload))
                            .expect("utf8 parse failed"),
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
