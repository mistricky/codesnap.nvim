use actix::{Actor, Context, Handler, Message, Recipient};
use rand::{rngs::ThreadRng, Rng};
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    event::Event,
    event_handler::{neovim::Neovim, Config},
};

type SessionID = usize;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<ServerMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConfigSetupMessage {
    pub msg: String,
}

pub struct Server {
    rng: ThreadRng,
    sessions: HashMap<SessionID, Recipient<ServerMessage>>,
    neovim: Arc<Mutex<Neovim>>,
    config: Option<Config>,
}

impl Server {
    pub fn new(neovim: Arc<Mutex<Neovim>>) -> Server {
        Server {
            rng: rand::thread_rng(),
            sessions: HashMap::new(),
            neovim,
            config: None,
        }
    }

    fn send_message_to_clients(&self, message: &str) {
        for (_, session) in &self.sessions {
            session.do_send(ServerMessage(message.to_string()))
        }
    }

    fn send_event_to_clients<T>(&self, event: Event<T>)
    where
        T: Serialize,
    {
        let stringify_event: String = event.into();
        self.send_message_to_clients(stringify_event.as_str())
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = SessionID;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen::<SessionID>();

        self.sessions.insert(id, msg.addr);

        if let Some(config) = &self.config {
            self.send_event_to_clients(Event::new("config_setup", config));
        }

        id
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ClientMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        self.send_message_to_clients(&msg.msg)
    }
}

impl Handler<ConfigSetupMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: ConfigSetupMessage, _: &mut Self::Context) -> Self::Result {
        let config = Config::from(msg.msg.as_str());

        self.config = Some(config.clone());
        self.send_event_to_clients(Event::new("config_setup", config));
    }
}
