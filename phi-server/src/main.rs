use crate::poker::DeckType;
use actix_session::CookieSession;
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use async_graphql::Schema;
use std::sync::Arc;
use std::time::Duration;
use structopt::StructOpt;
use uuid::Uuid;

mod cli;
mod gql;
mod poker;

// FIXME: need to rewrite BOTH spa impls so we can call `get_session_identity`
//  Probably refactor to follow the handler flow in `baked` and change how we
//  get raw bytes for a given filename depending on the feature.

#[cfg(feature = "baked")]
mod spa {
    use crate::gql::get_session_identity;
    use actix_session::Session;
    use actix_web::http::{header, StatusCode};
    use actix_web::{guard, web, HttpResponse, Responder};
    use include_dir::{include_dir, Dir};
    use std::path::PathBuf;

    static SPA_FILES: Dir = include_dir!("$PHI_STATIC_DIR");

    async fn handler(sess: Session, tail: web::Path<PathBuf>) -> impl Responder {
        // Try to establish player identity if not already set
        let _ = get_session_identity(&sess);
        let file = SPA_FILES
            .get_file(tail.into_inner())
            .or_else(|| SPA_FILES.get_file("index.html"))
            .expect("get baked file");

        let mime = file
            .path()
            .extension()
            .map(|s| s.to_ascii_lowercase())
            .and_then(|s| s.to_str().map(|x| x.to_string()))
            .map(|ext| actix_files::file_extension_to_mime(&ext))
            .unwrap_or(mime::APPLICATION_OCTET_STREAM);

        HttpResponse::build(StatusCode::OK)
            .insert_header((header::CONTENT_TYPE, mime))
            .body(file.contents())
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        log::info!("Configuring baked asset handler");
        cfg.service(web::resource("/{tail:.*}").guard(guard::Get()).to(handler));
    }
}

#[cfg(not(feature = "baked"))]
mod spa {
    use actix_files::NamedFile;
    use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
    use actix_web::web;
    use std::path::PathBuf;

    pub fn configure(cfg: &mut web::ServiceConfig) {
        let static_dir: PathBuf = std::env::var("PHI_STATIC_DIR")
            .ok()
            .map(Into::into)
            .unwrap_or_else(|| PathBuf::from("."));

        log::info!(
            "Configuring static asset handler using root={:?}",
            &static_dir
        );

        cfg.service(
            actix_files::Files::new("/", &static_dir)
                .index_file("index.html")
                .default_handler(fn_service(move |req: ServiceRequest| {
                    let static_dir = static_dir.clone();
                    async move {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async(static_dir.join("index.html")).await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    }
                })),
        );
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let opts: cli::Opt = cli::Opt::from_args();

    let admin_key = opts.admin_key.unwrap_or_else(|| Uuid::new_v4().to_string());

    log::info!("Admin Key: {}", &admin_key);
    log::info!("Server listening on {}", opts.http_addr);
    log::info!("Server listening on {}", opts.http_addr);

    let play_session = Arc::new(poker::PlaySession::new(
        admin_key,
        opts.deck_type,
        Duration::from_secs(opts.disconnect_timeout_secs),
    ));
    let poker_data = web::Data::new(play_session.clone());

    let schema = Schema::build(
        gql::model::Query,
        gql::model::Mutation,
        gql::model::Subscription,
    )
    .data(play_session.clone())
    .finish();

    let schema_data = web::Data::new(schema);

    // FIXME: configure via CLI/env
    let mut key = [0; 32];
    key[0] = 0;
    key[1] = 1;
    key[2] = 1;
    key[3] = 2;
    key[4] = 3;
    key[5] = 5;
    key[6] = 8;
    key[7] = 13;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                CookieSession::signed(&key)
                    .name("phi")
                    .secure(false)
                    .http_only(true)
                    .same_site(SameSite::Lax)
                    .path("/"),
            )
            .app_data(poker_data.clone())
            .app_data(schema_data.clone())
            .configure(gql::configure)
            .configure(spa::configure)
    })
    .bind(opts.http_addr)?
    .run()
    .await
}
