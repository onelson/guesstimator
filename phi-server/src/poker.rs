use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
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
    pub last_heartbeat: SystemTime,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            selected_card: None,
            last_heartbeat: SystemTime::now(),
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
