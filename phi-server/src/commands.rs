use crate::socket::{ConnectionId, PhiSocket};
use actix::{Addr, Message};
pub use phi_common::ClientCmd;
use phi_common::{GameState, PlayerId};

/// Messages to manipulate the shared state of the play session.
///
/// These messages are often going originate in socket handlers.
pub enum ServerCmd {
    AddConnection(ConnectionId, (PlayerId, Addr<PhiSocket>)),
    RemoveConnection(ConnectionId),
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
