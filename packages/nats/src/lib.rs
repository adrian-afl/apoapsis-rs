use async_nats::client::traits::Publisher;
use async_nats::message::OutboundMessage;
use async_nats::{ConnectOptions, HeaderMap};
use config::GLOBAL_CONFIG;
use futures_util::StreamExt;
use std::collections::VecDeque;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct IncomingRemoteIOMessage {
    pub name: String,
    pub payload: String,
    pub reply_to: Option<String>,
}

#[derive(Debug)]
pub struct OutgoingRemoteIOMessage {
    pub name: String,
    pub payload: String,
    pub success: bool,
}

pub struct NATSConnection {
    pub outbox: Arc<Mutex<VecDeque<OutgoingRemoteIOMessage>>>,
    pub inbox: Arc<Mutex<VecDeque<IncomingRemoteIOMessage>>>,
}

pub static NATS_CONNECTION: LazyLock<NATSConnection> = LazyLock::new(|| {
    let outbox: Arc<Mutex<VecDeque<OutgoingRemoteIOMessage>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    let inbox: Arc<Mutex<VecDeque<IncomingRemoteIOMessage>>> =
        Arc::new(Mutex::new(VecDeque::new()));

    // this needs to be done like that without send event macro
    // because here NATS_CONNECTION is not initialized and it would
    // go into a loop
    outbox.lock().unwrap().push_front(OutgoingRemoteIOMessage {
        name: "event.on_nats_connected".to_string(),
        payload: "null".to_string(),
        success: true,
    });

    connect_nats();

    NATSConnection { inbox, outbox }
});

// @api_event on_nats_connected()

#[macro_export]
macro_rules! send_event {
    ($name:expr) => {{
        $crate::NATS_CONNECTION
            .outbox
            .lock()
            .unwrap()
            .push_front($crate::OutgoingRemoteIOMessage {
                name: format!("event.{}", $name),
                payload: "null".to_string(),
                success: true,
            })
    }};
    ($name:expr, $payload:expr) => {{
        $crate::NATS_CONNECTION
            .outbox
            .lock()
            .unwrap()
            .push_front($crate::OutgoingRemoteIOMessage {
                name: format!("event.{}", $name),
                payload: serde_json::to_string(&$payload).unwrap(),
                success: true,
            })
    }};
}

fn connect_nats() {
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
            GLOBAL_CONFIG.nats_address.clone(),
            ConnectOptions::new().no_echo(),
        ))
        .unwrap();

    println!("Connected to NATS, subscribing to all...");

    let mut subscription = rt.block_on(client.subscribe(">")).unwrap();

    {
        let rt = rt.clone();
        thread::spawn(move || {
            println!("NATS transmit loop starting...");
            loop {
                let mut outbox = NATS_CONNECTION.outbox.lock().unwrap();
                while !outbox.is_empty() {
                    let message = outbox.pop_back().unwrap();
                    println!("OUTGOING {:?}", message);
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
                thread::sleep(Duration::from_millis(10));
            }
        });
    }
    {
        let rt = rt.clone();
        thread::spawn(move || {
            println!("NATS receive loop starting...");
            loop {
                if let Some(message) = rt.block_on(subscription.next()) {
                    println!("INCOMING {:?}", message);
                    let mut inbox = NATS_CONNECTION.inbox.lock().unwrap();
                    inbox.push_front(IncomingRemoteIOMessage {
                        name: message.subject.into_string(),
                        payload: String::from_utf8(Vec::from(message.payload))
                            .expect("utf8 parse failed"),
                        reply_to: message.reply.map(|x| x.to_string()),
                    })
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    println!("NATS connected and set up.");

    // You need to drop subscripions in async context, as they do spawn tasks to clean themselves up.
    // rt.block_on(async {
    //     drop(subscription);
    //     drop(client);
    // });
}
