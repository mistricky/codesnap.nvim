mod event;
mod event_handler;
mod port;
mod server;
mod session;

use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, Arbiter};
use actix_files::{Files, NamedFile};
use actix_web::{
    web::{self, Data, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use event_handler::neovim::Neovim;
use event_handler::EventHandler;
use port::get_available_port;
use server::Server;
use session::Session;

async fn index(
    req: HttpRequest,
    stream: Payload,
    server: Data<Arc<Addr<Server>>>,
) -> Result<HttpResponse, Error> {
    ws::start(Session::new(server), &req, stream)
}

async fn root() -> impl Responder {
    NamedFile::open_async("./public/index.html").await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let neovim = Arc::new(Mutex::new(Neovim::new()));
    let server = Arc::new(Server::new(neovim.clone()).start());
    let cloned_server = Arc::clone(&server);
    let cloned_neovim = neovim.clone();
    let available_port = get_available_port().unwrap();

    Arbiter::new().spawn(async move {
        EventHandler::new(cloned_neovim.clone(), cloned_server.clone(), available_port).start();
    });

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(index))
            .service(web::resource("/").to(root))
            .service(Files::new("/public", "./public"))
            .service(Files::new("/static", "./public/static"))
    })
    .bind(("127.0.0.1", available_port))?
    .run()
    .await;

    Ok(())
}
