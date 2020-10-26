use crate::gql::model::AdminKey;
use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use crossbeam::channel::{self, Receiver, Sender};
use phi_common::GameState;
use std::sync::Mutex;

pub async fn index(schema: web::Data<model::PokerSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/gql").subscription_endpoint("/gql"),
        )))
}

pub mod model;

type Channel<T> = (Sender<T>, Receiver<T>);

pub struct PlaySession {
    admin_key: AdminKey,
    game_state: Mutex<GameState>,
    /// Mutation queries can use this to request game state changes.    
    dispatch: Channel<()>,
    /// When the game state changes, this is used to notify subscribers.
    game_state_subscriptions: Mutex<Vec<Sender<GameState>>>,
}

impl PlaySession {
    pub fn new(admin_key: AdminKey) -> PlaySession {
        PlaySession {
            admin_key,
            game_state: Default::default(),
            dispatch: channel::unbounded(),
            game_state_subscriptions: Mutex::new(vec![]),
        }
    }

    /// Pushes the current `GameState` to all active subscriptions.
    fn notify_subscribers(&self) {
        let game_state = &*self.game_state.lock().unwrap();
        let mut subscribers = &mut *self.game_state_subscriptions.lock().unwrap();
        subscribers.retain(|tx| {
            // XXX: No idea if we'll see an error here if a subscription "ends."
            // The crossbeam docs mention the channels becoming "disconnected"
            // when either all senders or receivers drop, but it's not really
            // clear how the subscriptions terminate on the async-graphql side
            // of things. There's no hook to run code when the subscription ends.
            if let Err(e) = tx.send(game_state.clone()) {
                log::error!("{}", e);
                false
            } else {
                true
            }
        });
    }
}
