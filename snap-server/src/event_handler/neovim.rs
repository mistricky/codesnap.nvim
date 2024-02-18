use neovim_lib::{NeovimApi, Session, Value};
use std::sync::mpsc;

pub struct Neovim {
    pub instance: neovim_lib::Neovim,
}

impl Neovim {
    pub fn new() -> Neovim {
        let session = Session::new_parent().unwrap();
        let instance = neovim_lib::Neovim::new(session);

        Neovim { instance }
    }

    pub fn create_receiver(&mut self) -> mpsc::Receiver<(String, Vec<Value>)> {
        return self.instance.session.start_event_loop_channel();
    }

    pub fn print(&mut self, message: &str) {
        self.instance
            .command(&format!("lua print(\"{}\")", message))
            .unwrap();
    }
}
