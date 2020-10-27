use crate::gql::model::AdminKey;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response, WSSubscription};
use phi_common::GameState;
use std::sync::Mutex;
use tokio::sync::broadcast;

pub async fn index(schema: web::Data<model::PokerSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_ws(
    schema: web::Data<model::PokerSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(
        WSSubscription::new(Schema::clone(&*schema)),
        &["graphql-ws"],
        &req,
        payload,
    )
}

pub async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/gql").subscription_endpoint("/gql"),
        )))
}

pub mod model;

pub struct PlaySession {
    admin_key: AdminKey,
    game_state: Mutex<GameState>,
    /// When the game state changes, this is used to notify subscribers.
    game_state_notifier: broadcast::Sender<()>,
}

impl PlaySession {
    pub fn new(admin_key: AdminKey) -> PlaySession {
        let (tx, _rx) = broadcast::channel(100);
        PlaySession {
            admin_key,
            game_state: Default::default(),
            game_state_notifier: tx,
        }
    }

    /// Pushes the current `GameState` to all active subscriptions.
    fn notify_subscribers(&self) {
        self.game_state_notifier.send(()).unwrap();
    }
}
