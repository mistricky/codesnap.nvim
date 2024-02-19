pub mod arguments;
pub mod config;
pub mod messages;
pub mod neovim;

use actix::{Actor, Addr, AsyncContext, Context};
use arguments::parse_string_first;
pub use config::Config;
pub use messages::Message;
use neovim::Neovim;
use serde_json::{json, Value};
use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    event::Event,
    server::{ClientMessage, ConfigSetupMessage, Server},
};

pub struct EventHandler {
    neovim: Arc<Mutex<Neovim>>,
    server: Arc<Addr<Server>>,
}

impl Actor for EventHandler {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        self.start_listen();
    }
}

impl EventHandler {
    pub fn new(neovim: Arc<Mutex<Neovim>>, server: Arc<Addr<Server>>) -> EventHandler {
        EventHandler { neovim, server }
    }

    pub fn start_listen(&mut self) {
        let receiver = self.neovim.lock().unwrap().create_receiver();

        for (event_name, values) in receiver {
            self.neovim.lock().unwrap().print(&event_name);

            match Message::from(event_name.clone()) {
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
