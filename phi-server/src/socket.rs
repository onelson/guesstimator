use crate::commands::{ClientCmd, ServerCmd, SocketResponse};
use crate::play_session::PlaySession;
use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use log::error;
use phi_common::{PlayerId, ServerPush};

pub struct PhiSocket {
    player_id: PlayerId,
    play_session: Addr<PlaySession>,
}

impl PhiSocket {
    pub fn new(play_session: Addr<PlaySession>) -> PhiSocket {
        PhiSocket {
            play_session,
            player_id: PlayerId::new_v4(),
        }
    }
}

impl Actor for PhiSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PhiSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            match serde_json::from_str::<ClientCmd>(&text) {
                Ok(cmd) => {
                    self.play_session
                        .try_send(ServerCmd::Fwd(self.player_id, cmd))
                        .map_err(|e| error!("{}", e))
                        .ok();
                }
                Err(e) => error!("{}", e),
            }
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        self.play_session
            .try_send(ServerCmd::AddConnection(self.player_id, ctx.address()))
            .map_err(|e| error!("{}", e))
            .map(|_| {
                ctx.text(
                    serde_json::to_string(&ServerPush::PlayerRegistered {
                        player_id: self.player_id,
                    })
                    .unwrap(),
                );
            })
            .ok();
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        self.play_session
            .try_send(ServerCmd::RemoveConnection(self.player_id))
            .map_err(|e| error!("{}", e))
            .ok();
    }
}

impl Handler<SocketResponse> for PhiSocket {
    type Result = ();

    fn handle(&mut self, msg: SocketResponse, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SocketResponse::StateChange(new_state) => {
                ctx.text(serde_json::to_string(&ServerPush::StateChange { new_state }).unwrap())
            }
            SocketResponse::ConfirmAdminKey => {
                ctx.text(serde_json::to_string(&ServerPush::IsAdminUser).unwrap())
            }
        }
    }
}
