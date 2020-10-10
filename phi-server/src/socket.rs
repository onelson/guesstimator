use crate::commands::{ClientCmd, ServerCmd, StateChange};
use crate::play_session::PlaySession;
use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use log::{debug, error};
use phi_common::ServerPush;
use uuid::Uuid;

pub type ConnectionId = Uuid;

pub struct PhiSocket {
    connection_id: ConnectionId,
    play_session: Addr<PlaySession>,
}

impl PhiSocket {
    pub fn new(play_session: Addr<PlaySession>) -> PhiSocket {
        PhiSocket {
            play_session,
            connection_id: ConnectionId::new_v4(),
        }
    }
}

impl Actor for PhiSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PhiSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => match serde_json::from_str::<ClientCmd>(&text) {
                Ok(ClientCmd::RegisterPlayer(id)) => {
                    debug!("reg {}", id);
                    self.play_session
                        .try_send(ServerCmd::AddConnection(
                            self.connection_id,
                            (id, ctx.address()),
                        ))
                        .map_err(|e| error!("{}", e))
                        .unwrap();
                    self.play_session
                        .try_send(ClientCmd::RegisterPlayer(id))
                        .map_err(|e| error!("{}", e))
                        .unwrap();
                }
                Ok(cmd) => {
                    self.play_session
                        .try_send(cmd)
                        .map_err(|e| error!("{}", e))
                        .ok();
                }
                Err(e) => error!("{}", e),
            },
            _ => (),
        }
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        self.play_session
            .try_send(ServerCmd::RemoveConnection(self.connection_id))
            .map_err(|e| error!("{}", e))
            .ok();
    }
}

impl Handler<StateChange> for PhiSocket {
    type Result = ();

    fn handle(
        &mut self,
        StateChange(new_state): StateChange,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        ctx.text(serde_json::to_string(&ServerPush::StateChange { new_state }).unwrap());
    }
}
