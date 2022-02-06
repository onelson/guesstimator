use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::SystemTime;
use tokio::sync::broadcast;
use uuid::Uuid;

/// The names of the cards in the planning poker deck.
pub const FIB_DECK: [&str; 12] = [
    "0", "1", "2", "3", "5", "8", "13", "21", "100", "∞", "?", "☕",
];
pub const DAYS_DECK: [&str; 9] = ["0.5", "1", "1.5", "2", "3", "5", "∞", "?", "☕"];

#[derive(Debug)]
pub enum DeckType {
    Fibonacci,
    Days,
}

impl FromStr for DeckType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fib" | "" => Ok(DeckType::Fibonacci),
            "days" => Ok(DeckType::Days),
            _ => Err(format!("Invalid deck type: `{}`. Use `fib` or `days`.", s)),
        }
    }
}

/// Stable handle for identifying players, regardless of what the display name
/// is.
pub type PlayerId = Uuid;
/// Certain features are only enabled for players who know the secret key for
/// the session.
pub type AdminKey = String;

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
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct GameState {
    /// All the players in the planning poker game.
    pub players: HashMap<PlayerId, Player>,
    /// While "calling" player card selections are *frozen* and revealed to all
    /// players.
    pub is_calling: bool,
}

pub struct PlaySession {
    pub admin_key: AdminKey,
    pub game_state: Mutex<GameState>,
    /// When the game state changes, this is used to notify subscribers.
    pub game_state_notifier: broadcast::Sender<()>,
    pub deck: &'static [&'static str],
}

impl PlaySession {
    pub fn new(admin_key: AdminKey, deck_type: DeckType) -> PlaySession {
        let (tx, _rx) = broadcast::channel(100);
        PlaySession {
            admin_key,
            game_state: Default::default(),
            game_state_notifier: tx,
            deck: match deck_type {
                DeckType::Fibonacci => &FIB_DECK,
                DeckType::Days => &DAYS_DECK,
            },
        }
    }

    /// Pushes the current `GameState` to all active subscriptions.
    pub fn notify_subscribers(&self) {
        if let Err(err) = self.game_state_notifier.send(()) {
            log::warn!("{}", err);
        }
    }
}
