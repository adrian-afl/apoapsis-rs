pub mod lib_tcp;

use async_nats::client::traits::Publisher;
use futures_util::StreamExt;
use std::collections::VecDeque;
use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct IncomingRemoteIOMessage {
    pub name: String,
    pub payload: String,
    pub reply_to: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OutgoingRemoteIOMessage {
    pub name: String,
    pub payload: String,
    pub success: bool,
}

pub struct TCPControlServer {
    pub outbox: Arc<Mutex<VecDeque<OutgoingRemoteIOMessage>>>,
    pub inbox: Arc<Mutex<VecDeque<IncomingRemoteIOMessage>>>,
    pub current_stream: Arc<Mutex<Option<TcpStream>>>,
}

pub static TCP_CONTROL_SERVER: LazyLock<TCPControlServer> = LazyLock::new(|| {
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

    let current_stream = Arc::new(Mutex::new(None));

    let server = TCPControlServer {
        inbox: inbox.clone(),
        outbox: outbox.clone(),
        current_stream: current_stream.clone(),
    };

    {
        let current_stream = current_stream.clone();
        thread::spawn(move || {
            println!("TCP transmit loop starting...");
            loop {
                thread::sleep(Duration::from_millis(3));
                {
                    {
                        let stream = current_stream.lock().unwrap();
                        if stream.is_none() {
                            continue;
                        }
                    }
                    let mut outbox = outbox.lock().unwrap();
                    while !outbox.is_empty() {
                        let message = outbox.pop_back().unwrap();
                        let tmp = format!(
                            "{}\n{}\n{}\0",
                            &message.name,
                            &message.payload,
                            if message.success { "ok" } else { "error" },
                        );
                        let bytes = tmp.as_bytes();
                        let stream = current_stream.lock().unwrap();
                        stream
                            .as_ref()
                            .unwrap()
                            .write_all(bytes)
                            .expect("TCP transmit failed");
                    }
                }
            }
        });
    }
    {
        let current_stream = current_stream.clone();
        thread::spawn(move || {
            println!("TCP receive loop starting...");
            let mut buffer_big: Vec<u8> = Vec::new();
            let mut buffer_small: [u8; 1024] = [0u8; 1024];
            loop {
                thread::sleep(Duration::from_millis(3));
                // println!("TCP receive A...");
                {
                    let mut stream = current_stream.lock().unwrap();
                    if stream.is_none() {
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                }
                /*
                read until 0 is encountered
                */

                // println!("TCP receive B...");
                let n = {
                    // println!("A");
                    let mut stream = current_stream.lock().unwrap();
                    // println!("B");
                    let mut stream = stream.as_mut().unwrap();
                    // println!("C");
                    stream.read(&mut buffer_small)
                };
                // println!("D");
                match n {
                    Ok(n) => {
                        if n > 0 {
                            // println!(" >>>>> {n}");
                            buffer_big.extend(&buffer_small[0..n]);
                        }
                        if n == 0 {
                            //println!("Zero");
                            continue;
                        }
                    }
                    Err(e) => {
                        // println!(" >>>>> {}", e);
                        continue;
                    }
                }

                if buffer_big.len() > 0 {
                    let mut remaining = Vec::new();
                    let mut enumerated: Vec<_> = buffer_big
                        .split(|x| *x == 0x00u8)
                        .into_iter()
                        .enumerate()
                        .collect();

                    let len = enumerated.len();

                    for (i, slice) in enumerated {
                        if i == 0 && slice.len() == 0 {
                            // first slice empty means first byte as 0x00, so it can be ignored
                            continue;
                        }
                        if i == len - 1 {
                            // last one always to be used as remainder
                            // if can be empty which is very happy situation when the message ended at the read end
                            // if there are leftover it will contain it and it becomes the start of next received data
                            remaining.extend_from_slice(slice);
                            continue;
                        }
                        let mut message_items = slice.split(|x| *x == 0x0Au8);
                        // println!("{:?}", message_items);
                        let name =
                            String::from_utf8_lossy(message_items.next().unwrap()).to_string();
                        let reply_to =
                            String::from_utf8_lossy(message_items.next().unwrap()).to_string();
                        let payload =
                            String::from_utf8_lossy(message_items.next().unwrap()).to_string();

                        let mut inbox = TCP_CONTROL_SERVER.inbox.lock().unwrap();
                        inbox.push_front(IncomingRemoteIOMessage {
                            name,
                            reply_to: if reply_to.len() > 0 {
                                Some(reply_to)
                            } else {
                                None
                            },
                            payload,
                        })
                    }
                    buffer_big = remaining;
                }
            }
        });
    }
    {
        let current_stream = current_stream.clone();
        thread::spawn(move || {
            println!("TCP stream set loop starting...");
            let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                stream.set_nonblocking(true).expect("TODO: panic message");

                // stream
                //     .set_read_timeout(Some(Duration::from_millis(5)))
                //     .expect("TODO: panic message");
                //
                // stream
                //     .set_write_timeout(Some(Duration::from_millis(5)))
                //     .expect("TODO: panic message");

                let mut current = current_stream.lock().unwrap();
                println!("TCP stream connected");
                *current = Some(stream);
                println!("Stream set {:?}...", *current);
            }
        });
    }

    println!("TCP LISTENING");

    server
});

// @api_event on_nats_connected()

#[macro_export]
macro_rules! send_event {
    ($name:expr) => {{
        $crate::TCP_CONTROL_SERVER
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
        $crate::TCP_CONTROL_SERVER
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
