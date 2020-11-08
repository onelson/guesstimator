use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use uuid::Uuid;

/// The names of the cards in the planning poker deck.
pub const CARDS: [&str; 12] = [
    "0", "1", "2", "3", "5", "8", "13", "21", "100", "∞", "?", "☕",
];

/// Stable handle for identifying players, regardless of what the display name
/// is.
pub type PlayerId = Uuid;
/// Certain features are only enabled for players who know the secret key for
/// the session.
pub type AdminKey = Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Player {
    /// The name displayed with the cards.
    pub name: String,
    /// Index into the card data, `CARDS`.
    pub selected_card: Option<usize>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            selected_card: None,
        }
    }

    pub fn selected_card_name(&self) -> Option<&'static str> {
        self.selected_card.map(|idx| self::CARDS[idx])
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct GameState {
    /// All the players in the planning poker game.
    pub players: HashMap<PlayerId, Player>,
    /// While "calling" player card selections are *frozen* and revealed to all
    /// players.
    pub is_calling: bool,
}

/// Messages the server will receive from connected clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientCmd {
    SetPlayerName(String),
    SetPlayerCard(Option<usize>),
    RemovePlayer,
    /// Clients send their key (if they have one) and the server validates it.
    /// If the key is valid, the server will respond with
    /// `ServerPush::IsAdminUser`.
    AdminChallenge(AdminKey),
    Call,
    Resume,
    Reset,
}

/// Used to push messages to connected websocket clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerPush {
    /// Sent back to a client as soon as the connection has been established.
    PlayerRegistered { player_id: PlayerId },
    /// Broadcast a new game state.
    StateChange { new_state: GameState },
    /// Confirm that a client knows the correct admin key.
    IsAdminUser,
}
