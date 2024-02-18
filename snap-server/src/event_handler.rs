mod messages;
pub mod neovim;

use actix::{Actor, Addr, AsyncContext, Context};
pub use messages::Message;
use neovim::Neovim;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    event::Event,
    server::{ClientMessage, Server},
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
            match Message::from(event_name.clone()) {
                Message::PreviewCode => self.server.do_send(ClientMessage {
                    msg: Event::new(
                        "hello".to_string(),
                        serde_json::from_str::<String>(
                            values
                                .iter()
                                .map(|value| value.to_string())
                                .collect::<Vec<String>>()
                                .first()
                                .unwrap(),
                        )
                        .unwrap(),
                    )
                    .into(),
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
