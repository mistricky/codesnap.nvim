pub mod arguments;
pub mod config;
pub mod messages;
pub mod neovim;

use actix::{Actor, Addr, Context};
use arguments::parse_string_first;
pub use config::Config;
use headless_chrome::{protocol::cdp::Page, Browser, Tab};
pub use messages::Message;
use neovim::Neovim;
use serde_json::{json, Value};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

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

    pub fn open_browser(&self) -> Result<Arc<Tab>, Box<dyn Error>> {
        let browser = Browser::default()?;

        let tab = browser.new_tab()?;

        tab.navigate_to(format!("http://localhost:{}", self.port).as_str())?;

        Ok(tab)
    }

    pub fn start_listen(&mut self) {
        let receiver = self.neovim.lock().unwrap().create_receiver();

        for (event_name, values) in receiver {
            // self.neovim.lock().unwrap().print(&event_name);

            match Message::from(event_name.clone()) {
                Message::OpenPreview => {
                    let _ = webbrowser::open(format!("http://localhost:{}", self.port).as_str());
                }
                Message::Copy => {
                    // let _png_data = tab
                    //     .wait_for_element("#frame")
                    //     .unwrap()
                    //     .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)
                    //     .unwrap();
                    //
                    // std::fs::write("hello_world.png", _png_data).unwrap();

                    //     self.server.do_send(ClientMessage {
                    //     msg: Event::new("copy", json!("{}")).into(),
                    // })
                }
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
