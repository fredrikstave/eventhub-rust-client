use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

pub struct EventhubOptions {
    ping_interval: u32,
    ping_timeout: u32,
    max_failed_pings: u8,
    reconnect_interval: u32,
    disable_ping_check: bool
}

struct Eventhub {
  url: String,
  token: String,
  options: EventhubOptions
}

impl Eventhub {
    pub fn new(url: &str, token: &str, options: Option<EventhubOptions>) -> Eventhub {
        match options {
            Some(options) => Eventhub {
                url: String::from(url),
                token: String::from(token),
                options: options
            },
            None => Eventhub {
                url: String::from(url),
                token: String::from(token),
                options: EventhubOptions {
                    ping_interval: 10000,
                    ping_timeout: 3000,
                    max_failed_pings: 3,
                    reconnect_interval: 10000,
                    disable_ping_check: false
                }
            },
        }
    }

    pub fn connect(&self) {
        let server = TcpListener::bind(&self.url).unwrap();
        for stream in server.incoming() {
            spawn (move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                loop {
                    let msg = websocket.read_message().unwrap();

                    // We do not want to send back ping/pong messages.
                    if msg.is_binary() || msg.is_text() {
                        websocket.write_message(msg).unwrap();
                    }
                }
            });
        }
    }

    pub fn publish(&self, topic: String, payload: String) {

    }

    pub fn subscribe(&self, topic: String) {

    }

    pub fn unsubscribe(&self, topic: String) {

    }

    pub fn list_subscription(&self) {

    }

    pub fn list_all_subscriptions(&self) {

    }
}
