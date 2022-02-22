use crate::poker::PlayerId;
use actix_session::Session;
use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use std::sync::Arc;

pub mod model;

#[derive(Clone, Debug)]
pub struct SessionIdentity {
    name: String,
    id: PlayerId,
}

pub fn get_session_identity(session: &Session) -> SessionIdentity {
    let id: PlayerId = {
        let sess_player_id = session.get::<PlayerId>("player_id").unwrap();
        match sess_player_id {
            None => {
                let id = PlayerId::new_v4();
                log::debug!("player id not in request, setting id={id}");
                session.insert("player_id", id).unwrap();
                id
            }
            Some(id) => {
                log::trace!("player id in request: {}", &id);
                id
            }
        }
    };
    let name: String = {
        let sess_player_name = session.get::<String>("player_name").unwrap();
        match sess_player_name {
            None => {
                let name = String::from("Guest");
                log::debug!("player name not present in request, setting name={name}");
                session.insert("player_name", name.clone()).unwrap();
                name
            }
            Some(name) => {
                log::trace!("player name in request: {}", &name);

                name
            }
        }
    };
    SessionIdentity { id, name }
}

async fn index(
    session: Session,
    poker: web::Data<Arc<crate::poker::PlaySession>>,
    schema: web::Data<model::PokerSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    let identity = get_session_identity(&session);
    let req = req.data(identity.clone());
    let resp = schema.execute(req).await.into();

    {
        let poker = poker.into_inner();
        let gstate = poker.game_state.lock().unwrap();
        if let Some(player) = gstate.players.get(&identity.id) {
            if &player.name != &identity.name {
                log::debug!(
                    "Player name change detected: id={} old name={} new name={}",
                    &identity.id,
                    &identity.name,
                    &player.name,
                );
                if let Err(e) = session.insert("player_name", &player.name) {
                    log::error!("{e}");
                }
            }
        }
    }
    resp
}

async fn index_ws(
    schema: web::Data<model::PokerSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/gql").subscription_endpoint("/gql"),
        )))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/gql").guard(guard::Post()).to(index))
        .service(
            web::resource("/gql")
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws),
        )
        .service(
            web::resource("/gql-playground")
                .guard(guard::Get())
                .to(index_playground),
        );
}
