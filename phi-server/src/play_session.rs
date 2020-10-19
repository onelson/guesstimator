use crate::commands::{ClientCmd, ServerCmd, SocketResponse};
use crate::socket::PhiSocket;
use actix::{Actor, Addr, Context, Handler};
use log::{debug, error, warn};
use phi_common::{AdminKey, GameState, Player, PlayerId};
use std::collections::HashMap;

/// Holds the state of the play session including which players are holding
/// which cards and whether or not the owner of the session is "calling."
///
/// `Addr`s for each player's websocket connection are also stored here in order
/// to synchronize client states when the game state changes.
#[derive(Default)]
pub struct PlaySession {
    /// Used to check to see if a connected client has extra privs or not.
    admin_key: AdminKey,
    /// Inventory of incoming WS connections, and which players are "on the line."
    sockets: HashMap<PlayerId, Addr<PhiSocket>>,
    /// Shared data to track the names and card selections of each player.
    game_state: GameState,
}

impl PlaySession {
    pub fn new(admin_key: AdminKey) -> PlaySession {
        PlaySession {
            admin_key,
            ..PlaySession::default()
        }
    }

    /// Pushes the current `GameState` to all active connections.
    fn notify_clients(&self) {
        for addr in self.sockets.values() {
            addr.try_send(SocketResponse::StateChange(self.game_state.clone()))
                .map_err(|e| error!("{}", e))
                .ok();
        }
    }
}

impl Actor for PlaySession {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("stopped");
    }
}

impl Handler<ServerCmd> for PlaySession {
    type Result = ();

    fn handle(&mut self, msg: ServerCmd, _ctx: &mut Context<Self>) -> Self::Result {
        let orig = self.game_state.clone();

        match msg {
            ServerCmd::AddConnection(player_id, addr) => {
                debug!("Adding connection `{}`", player_id);
                self.sockets.insert(player_id, addr);
                self.game_state
                    .players
                    .insert(player_id, Player::new(String::from("Guest")));
            }
            ServerCmd::RemoveConnection(player_id) => {
                debug!("Removing connection `{}`", player_id);
                self.sockets.remove(&player_id);
                self.game_state.players.remove(&player_id);
            }
            ServerCmd::Fwd(player_id, cmd) => {
                debug!("cmd=`{:?}`", &cmd);

                match cmd {
                    ClientCmd::SetPlayerName(name) => {
                        if let Some(player) = self.game_state.players.get_mut(&player_id) {
                            player.name = name;
                        } else {
                            warn!("Tried to update name for unknown player: `{}`", player_id);
                        }
                    }
                    ClientCmd::RemovePlayer => {
                        self.game_state.players.remove(&player_id);
                    }
                    ClientCmd::Call => {
                        self.game_state.is_calling = true;
                    }
                    ClientCmd::Resume => {
                        self.game_state.is_calling = false;
                    }
                    ClientCmd::SetPlayerCard(card) => {
                        if let Some(player) = self.game_state.players.get_mut(&player_id) {
                            match player.selected_card.take() {
                                prev if prev == card => (),
                                _ => player.selected_card = card,
                            }
                        }
                    }
                    ClientCmd::AdminChallenge(admin_key) => {
                        if let Some(addr) = self.sockets.get(&player_id) {
                            if self.admin_key == admin_key {
                                addr.try_send(SocketResponse::ConfirmAdminKey)
                                    .map_err(|e| error!("{}", e))
                                    .ok();
                            } else {
                                warn!("Player `{}` challenged with invalid key.", player_id);
                            }
                        }
                    }
                    ClientCmd::Reset => {
                        for mut player in self.game_state.players.values_mut() {
                            player.selected_card = None;
                        }
                        self.game_state.is_calling = false;
                    }
                }
            }
        }

        if orig != self.game_state {
            debug!("Notifying clients");
            self.notify_clients();
        }
    }
}
