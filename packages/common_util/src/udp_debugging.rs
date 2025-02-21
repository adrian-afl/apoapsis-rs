use std::borrow::ToOwned;
use std::net::UdpSocket;
use std::sync::{LazyLock, Mutex};

pub struct UDPDebugging {
    socket: UdpSocket,
    target_address: Mutex<String>,
    enabled: Mutex<bool>,
}

// TODO rethink this monstrosity
impl UDPDebugging {
    pub fn set_target(&self, target_address: &str) {
        *self.target_address.lock().unwrap() = target_address.to_owned();
    }

    pub fn send(&self, data: &str) {
        if *self.enabled.lock().unwrap() {
            self.socket
                .send_to(
                    data.as_bytes(),
                    self.target_address.lock().unwrap().as_str(),
                )
                .unwrap();
        }
    }
}

pub static UDP_DEBUGGING: LazyLock<UDPDebugging> = LazyLock::new(|| UDPDebugging {
    socket: UdpSocket::bind("127.0.0.1:0").unwrap(),
    target_address: Mutex::new("127.0.0.1:6000".to_owned()),
    enabled: Mutex::new(true),
});

#[macro_export]
macro_rules! udebug {
    ($fmt_str:literal) => {{
        #[cfg(debug_assertions)]
        $crate::udp_debugging::UDP_DEBUGGING.send(&format!("{}:{}: {}", file!(), line!(), $fmt_str));
    }};

    ($fmt_str:literal, $($args:expr),*) => {{
        #[cfg(debug_assertions)]
        $crate::udp_debugging::UDP_DEBUGGING.send(&format!("{}:{}: {}", file!(), line!(), &format!($fmt_str, $($args),*)));
    }};
}
