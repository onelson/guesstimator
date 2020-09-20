use std::collections::HashMap;
use uuid::Uuid;
use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

/// The names of the cards in the planning poker deck.
const CARDS: [&str; 12] = [
    "0", "1", "2", "3", "5", "8", "13", "21", "100", "∞", "?", "☕",
];
/// Stable handle for identifying players, regardless of what the display name
/// is.
pub type PlayerId = Uuid;
/// Certain features are only enabled for players who know the secret key for
/// the session.
pub type AdminKey = Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Player {
    /// The name displayed with the cards.
    pub name: String,
    /// Index into the card data, `PlanningPokerStore::cards`.
    pub selected_card: Option<usize>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name.into(),
            selected_card: None,
        }
    }

    pub fn next_id() -> PlayerId {
        Uuid::new_v4()
    }
}

#[derive(Debug)]
pub struct PlanningPokerStore {
    /// All the players in the planning poker game.
    pub players: HashMap<PlayerId, Player>,
    /// While "calling" player card selections are *frozen* and revealed to all
    /// players.
    pub is_calling: bool,
    /// Whether or not the client knows the secret key for the session.
    pub is_admin: bool,
}

impl PlanningPokerStore {
    pub const CARDS: [&'static str; 12] = CARDS;
}

#[derive(Debug)]
pub enum Request {
    InitClient(PlayerId),
    InitAdminClient(PlayerId),
    /// Register a new player.
    AddPlayer(Uuid, String),
    /// Change a player's name.
    RenamePlayer(PlayerId, String),
    /// Select or deselect a card.
    ChangePlayerCard(PlayerId, Option<usize>),
    /// Player removed from the game.
    RemovePlayer(PlayerId),
    /// Game is "paused" and all player cards are shown.
    Call,
    /// Game continues, players can alter their selections while cards are hidden.
    Resume,
    /// Same as "resume" but all player card selections are cleared.
    Reset,
}

impl Store for PlanningPokerStore {
    // For the time being, it seems like Input and Action can be the same thing.
    type Input = Request;
    type Action = Request;

    fn new() -> Self {
        PlanningPokerStore {
            players: HashMap::new(),
            is_calling: false,
            is_admin: false,
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        link.send_message(msg);
    }

    fn reduce(&mut self, msg: Self::Action) {
        use Request::*;
        match msg {
            InitClient(_id) => {}
            InitAdminClient(_id) => {
                self.is_admin = true;
            }
            AddPlayer(id, name) => {
                self.players.insert(id, Player::new(name));
            }
            RenamePlayer(id, name) => {
                if let Some(player) = self.players.get_mut(&id) {
                    player.name = name;
                }
            }
            ChangePlayerCard(id, selection) => {
                if let Some(player) = self.players.get_mut(&id) {
                    match player.selected_card.take() {
                        prev if prev == selection => (),
                        _ => player.selected_card = selection,
                    }
                }
            }
            RemovePlayer(id) => {
                self.players.remove(&id);
            }
            Call => {
                self.is_calling = true;
            }
            Resume => {
                self.is_calling = false;
            }
            Reset => {
                for mut player in self.players.values_mut() {
                    player.selected_card = None;
                }
                self.is_calling = false;
            }
        }
    }
}
