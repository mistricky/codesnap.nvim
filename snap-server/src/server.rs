use actix::{Actor, Context, Handler, Message, Recipient};
use rand::{rngs::ThreadRng, Rng};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::event_handler::neovim::Neovim;

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

pub struct Server {
    rng: ThreadRng,
    sessions: HashMap<SessionID, Recipient<ServerMessage>>,
    neovim: Arc<Mutex<Neovim>>,
}

impl Server {
    pub fn new(neovim: Arc<Mutex<Neovim>>) -> Server {
        Server {
            rng: rand::thread_rng(),
            sessions: HashMap::new(),
            neovim,
        }
    }

    fn send_message_to_clients(&self, message: &str) {
        for (_, session) in &self.sessions {
            session.do_send(ServerMessage(message.to_string()))
        }
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
