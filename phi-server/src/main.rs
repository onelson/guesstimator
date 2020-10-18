use crate::play_session::PlaySession;
use actix::{Actor, Addr};
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use log::debug;
use socket::PhiSocket;
use std::path::PathBuf;
use uuid::Uuid;

mod commands;
mod play_session;
mod socket;

async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
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
    let static_dir: PathBuf = std::env::var("PHI_STATIC_DIR")
        .ok()
        .map(Into::into)
        .unwrap_or_else(|| PathBuf::from("."));
    println!("\nUsing Static Dir: {:?}\n", &static_dir);
    println!("\nAdmin Key: {}\n", &admin_key);

    let play_session = PlaySession::create(|_| {
        debug!("init play session");
        PlaySession::new(admin_key)
    });

    HttpServer::new(move || {
        // Each worker needs its own copy of the Addr.
        let play_session = play_session.clone();
        let static_dir = static_dir.clone();
        App::new()
            .wrap(Logger::default())
            .app_data(Data { play_session })
            .route("/ws", web::get().to(ws))
            .service(actix_files::Files::new("/", static_dir).index_file("index.html"))
    })
    .bind("0.0.0.0:7878")?
    .run()
    .await
}
