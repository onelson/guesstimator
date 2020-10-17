use crate::play_session::PlaySession;
use actix::{Actor, Addr};
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use log::debug;
use socket::PhiSocket;
use uuid::Uuid;

mod commands;
mod play_session;
mod socket;

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let data = req.app_data::<Data>().unwrap();
    let game_state = data.play_session.clone();
    ws::start(PhiSocket::new(game_state), &req, stream)
}

struct Data {
    play_session: Addr<PlaySession>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let admin_key = Uuid::new_v4();
    println!("\nAdmin Key: {}\n", &admin_key);

    let play_session = PlaySession::create(|_| {
        debug!("init play session");
        PlaySession::new(admin_key)
    });

    HttpServer::new(move || {
        // Each worker needs its own copy of the Addr.
        let play_session = play_session.clone();
        App::new()
            .wrap(Logger::default())
            .app_data(Data { play_session })
            .route("/ws", web::get().to(index))
    })
    .bind("0.0.0.0:7878")?
    .run()
    .await
}
