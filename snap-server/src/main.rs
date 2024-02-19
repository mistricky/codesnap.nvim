mod event;
mod event_handler;
mod server;
mod session;

use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, Arbiter, StreamHandler};
use actix_web::{
    web::{self, Data, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use event_handler::{neovim::Neovim, Config};
use event_handler::{EventHandler, Message};
use server::Server;
use session::Session;

async fn index(
    req: HttpRequest,
    stream: Payload,
    server: Data<Arc<Addr<Server>>>,
) -> Result<HttpResponse, Error> {
    ws::start(Session::new(server), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let neovim = Arc::new(Mutex::new(Neovim::new()));
    let server = Arc::new(Server::new(neovim.clone()).start());
    let cloned_server = Arc::clone(&server);

    Arbiter::new().spawn(async {
        EventHandler::new(neovim, cloned_server).start();
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn main() {
//     let data = r#"
//         {
//             "breadcrumbs":true,
//             "watermark":"CodeSnap.nvim",
//             "mac_window_bar":true,
//             "column_number":true,
//             "auto_load":true,
//             "background":{
//                 "grandient":true
//             }
//         }"#;
//
//     let config: Config = serde_json::from_str(data).unwrap();
//
//     println!("{:?}", config)
// }
