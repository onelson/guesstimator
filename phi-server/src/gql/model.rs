//! Graphql has some modelling constraints that conflict with the original
//! design used for the websocket version, so I'm redefining a bunch of the
//! types used for the game here.

use crate::poker::{AdminKey, PlayerId};
use async_graphql::*;
use std::time::{Duration, SystemTime};
use tokio_stream::{self as stream, wrappers::BroadcastStream, Stream, StreamExt};

/// Players that fail to send a heartbeat within this time will be dropped from
/// the game.
const MAX_PLAYER_IDLE: Duration = Duration::from_secs(10);

pub type PokerSchema = Schema<Query, Mutation, Subscription>;

#[derive(Clone, Debug, PartialEq, SimpleObject)]
struct Player {
    pub id: PlayerId,
    /// The name displayed with the cards.
    pub name: String,
    /// Index into the card data, `CARDS`.
    pub selected_card: Option<i32>,
}

impl From<(&PlayerId, &crate::poker::Player)> for Player {
    fn from((id, others): (&PlayerId, &crate::poker::Player)) -> Self {
        Player {
            id: *id,
            name: others.name.clone(),
            selected_card: others.selected_card.map(|n| n as i32),
        }
    }
}

struct GameState;

#[Object]
impl GameState {
    async fn is_calling(&self, ctx: &Context<'_>) -> bool {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        let game_state = session.game_state.lock().unwrap();
        game_state.is_calling
    }

    async fn players(&self, ctx: &Context<'_>) -> Vec<Player> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        let game_state = session.game_state.lock().unwrap();
        game_state.players.iter().map(Into::into).collect()
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn cards(&self, ctx: &Context<'_>) -> &[&str] {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        session.deck
    }

    async fn game_state(&self) -> GameState {
        GameState
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register(&self, ctx: &Context<'_>) -> Result<PlayerId> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        let player_id = PlayerId::new_v4();
        {
            let mut game_state = session.game_state.lock().unwrap();
            game_state
                .players
                .insert(player_id, crate::poker::Player::new(String::from("Guest")));
        }
        session.notify_subscribers();
        Ok(player_id)
    }

    /// Clients that want admin privileges send their key.
    /// The bool return is for if the keys match or not.
    async fn admin_challenge(&self, ctx: &Context<'_>, key: AdminKey) -> Result<bool> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        Ok(session.admin_key == key)
    }

    async fn heartbeat(&self, ctx: &Context<'_>, player_id: PlayerId) -> Result<bool> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        {
            let mut state = session.game_state.lock().unwrap();
            if let Some(player) = state.players.get_mut(&player_id) {
                player.last_heartbeat = SystemTime::now();
            } else {
                log::warn!(
                    "Tried to update heartbeat for unknown player: `{}`",
                    player_id
                );
            }
            let prev_players = state.players.clone();

            state
                .players
                .retain(|_k, v| v.last_heartbeat.elapsed().unwrap() < MAX_PLAYER_IDLE);
            if state.players != prev_players {
                log::warn!(
                    "removing idle players: {}",
                    prev_players.len() - state.players.len()
                );
                session.notify_subscribers();
            }
        }
        Ok(true)
    }

    async fn set_player_name(
        &self,
        ctx: &Context<'_>,
        player_id: PlayerId,
        name: String,
    ) -> Result<bool> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        let outcome = {
            let mut game_state = session.game_state.lock().unwrap();
            if let Some(player) = game_state.players.get_mut(&player_id) {
                player.name = name;
                Ok(true)
            } else {
                log::warn!("Tried to update name for unknown player: `{}`", player_id);
                Ok(false)
            }
        };
        session.notify_subscribers();
        outcome
    }

    async fn set_player_card(
        &self,
        ctx: &Context<'_>,
        player_id: PlayerId,
        card: Option<i32>,
    ) -> Result<bool> {
        let card = card.map(|n| n as usize);
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        let outcome = {
            let mut game_state = session.game_state.lock().unwrap();
            if game_state.is_calling {
                return Err(Error::new(
                    "Game is currently calling. Selections are locked.",
                ));
            }
            if let Some(player) = game_state.players.get_mut(&player_id) {
                match player.selected_card.take() {
                    prev if prev == card => (),
                    _ => player.selected_card = card,
                }
                Ok(true)
            } else {
                log::warn!("Tried to update card for unknown player: `{}`", player_id);
                Ok(false)
            }
        };
        session.notify_subscribers();
        outcome
    }

    async fn remove_player(&self, ctx: &Context<'_>, player_id: PlayerId) -> Result<bool> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        {
            let mut game_state = session.game_state.lock().unwrap();
            game_state.players.remove(&player_id);
        }
        session.notify_subscribers();
        Ok(true)
    }

    async fn call(&self, ctx: &Context<'_>) -> Result<bool> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        {
            let mut game_state = session.game_state.lock().unwrap();
            game_state.is_calling = true;
        }
        session.notify_subscribers();
        Ok(true)
    }

    async fn resume(&self, ctx: &Context<'_>) -> Result<bool> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        {
            let mut game_state = session.game_state.lock().unwrap();
            game_state.is_calling = false;
        }
        session.notify_subscribers();
        Ok(true)
    }

    async fn reset(&self, ctx: &Context<'_>) -> Result<bool> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        {
            let mut game_state = session.game_state.lock().unwrap();
            for mut player in game_state.players.values_mut() {
                player.selected_card = None;
            }
            game_state.is_calling = false;
        }
        session.notify_subscribers();
        Ok(true)
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn game_state(&self, ctx: &Context<'_>) -> impl Stream<Item = GameState> {
        let session = ctx.data_unchecked::<crate::poker::PlaySession>();
        let rx = BroadcastStream::new(session.game_state_notifier.subscribe());
        // Who knows when the next game state change will happen, so seed the
        // stream with one message to kick things off.
        let init = stream::iter(vec![GameState]);
        // Additional game states will flow over the socket with each time a
        // mutation happens (ie, when `notify_subscribers()` is called).
        init.merge(rx.map(|_| GameState))
    }
}
