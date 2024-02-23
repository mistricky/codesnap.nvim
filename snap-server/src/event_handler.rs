pub mod arguments;
pub mod config;
pub mod messages;
pub mod neovim;

use actix::{Actor, Addr, Context};
use arguments::parse_string_first;
pub use config::Config;
pub use messages::Message;
use neovim::Neovim;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use crate::{
    event::Event,
    server::{ClientMessage, ConfigSetupMessage, Server},
};

pub struct EventHandler {
    neovim: Arc<Mutex<Neovim>>,
    server: Arc<Addr<Server>>,
    port: u16,
}

impl Actor for EventHandler {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        self.start_listen();
    }
}

impl EventHandler {
    pub fn new(neovim: Arc<Mutex<Neovim>>, server: Arc<Addr<Server>>, port: u16) -> EventHandler {
        EventHandler {
            neovim,
            server,
            port,
        }
    }

    pub fn start_listen(&mut self) {
        let receiver = self.neovim.lock().unwrap().create_receiver();

        for (event_name, values) in receiver {
            match Message::from(event_name.clone()) {
                Message::OpenPreview => {
                    let _ = webbrowser::open(format!("http://localhost:{}", self.port).as_str());
                }
                // The Copy message is not implement yet, but will support in the future
                Message::Copy => {}
                Message::PreviewCode => self.server.do_send(ClientMessage {
                    msg: Event::new(
                        "code",
                        serde_json::from_str::<Value>(parse_string_first(&values).as_str())
                            .unwrap(),
                    )
                    .into(),
                }),
                Message::ConfigSetup => self.server.do_send(ConfigSetupMessage {
                    msg: parse_string_first(&values),
                }),
                Message::Unknown => self
                    .neovim
                    .lock()
                    .unwrap()
                    .print(&format!("Unhandled event which name is {}", &event_name)),
            };
        }
    }
}
