use crate::socket::PhiSocket;
use actix::{Addr, Message};
pub use phi_common::ClientCmd;
use phi_common::{GameState, PlayerId};

/// Messages to manipulate the shared state of the play session.
///
/// These messages are often going originate in socket handlers.
pub enum ServerCmd {
    AddConnection(PlayerId, Addr<PhiSocket>),
    RemoveConnection(PlayerId),
    /// When the client sends commands to the server, the server needs to add
    /// the `PlayerId` to the message.
    Fwd(PlayerId, ClientCmd),
}

impl Message for ServerCmd {
    type Result = ();
}

pub enum SocketResponse {
    StateChange(GameState),
    ConfirmAdminKey,
}

impl Message for SocketResponse {
    type Result = ();
}
