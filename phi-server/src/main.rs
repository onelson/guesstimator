use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};
use async_graphql::Schema;
use std::path::PathBuf;
use uuid::Uuid;

mod gql;
mod poker;

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

    let schema = Schema::build(
        gql::model::Query,
        gql::model::Mutation,
        gql::model::Subscription,
    )
    .data(gql::PlaySession::new(admin_key))
    .finish();

    HttpServer::new(move || {
        let static_dir = static_dir.clone();
        App::new()
            .wrap(Logger::default())
            .data(schema.clone())
            .service(web::resource("/gql").guard(guard::Post()).to(gql::index))
            .service(
                web::resource("/gql")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(gql::index_ws),
            )
            .service(
                web::resource("/gql-playground")
                    .guard(guard::Get())
                    .to(gql::index_playground),
            )
            .service(actix_files::Files::new("/", static_dir).index_file("index.html"))
    })
    .bind("0.0.0.0:7878")?
    .run()
    .await
}
