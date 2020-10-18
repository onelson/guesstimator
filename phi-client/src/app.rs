use crate::player_cards::PlayerCards;
use crate::text_edit::TextEdit;
use log::{debug, warn};
use phi_common::{AdminKey, ClientCmd, Player, PlayerId, ServerPush, CARDS};
use std::collections::HashMap;
use uuid::Uuid;
use web_sys::UrlSearchParams;
use yew::callback::Callback;
use yew::format::Text;
use yew::prelude::*;
use yew::services::websocket::{WebSocketStatus, WebSocketTask};
use yew::services::WebSocketService;
use yewtil::NeqAssign;

pub struct App {
    link: ComponentLink<Self>,
    client_id: PlayerId,
    admin_key: Option<AdminKey>,
    is_admin: bool,
    players: HashMap<PlayerId, Player>,
    is_calling: bool,
    socket: WebSocketTask,
}

#[derive(Debug)]
pub enum Msg {
    SelectCard(usize),
    SetPlayerName(String),
    ToggleCalling,
    SocketRecv(String),
    SocketStatus(WebSocketStatus),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let window = yew::utils::window();

        let ws_uri = {
            let scheme = if window.location().protocol().unwrap() == "https:" {
                "wss:"
            } else {
                "ws:"
            };
            format!("{}//{}/ws", scheme, window.location().host().unwrap())
        };

        let socket = WebSocketService::connect_text(
            &ws_uri,
            link.callback(|text: Text| Msg::SocketRecv(text.unwrap())), // FIXME
            link.callback(|status| Msg::SocketStatus(status)),
        )
        .unwrap();

        let client_id = Uuid::new_v4(); // FIXME: get from server?

        let admin_key = {
            match window.location().search() {
                Ok(search) => UrlSearchParams::new_with_str(&search)
                    .map_err(|_| warn!("Failed to parse search params."))
                    .ok()
                    .and_then(|params| params.get("key"))
                    .and_then(|s| s.parse().ok()),
                _ => None,
            }
        };

        debug!("key? `{:?}`", admin_key);

        App {
            link,
            client_id,
            admin_key,
            // start out as false, but flip to true if the admin key passes validation.
            is_admin: false,
            players: Default::default(),
            is_calling: false,
            socket,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SocketStatus(status) => {
                debug!("ws status={:?}", status);
                if status == WebSocketStatus::Opened {
                    self.socket.send(
                        serde_json::to_string(&ClientCmd::RegisterPlayer(self.client_id))
                            .map_err(Into::into),
                    );
                    if let Some(admin_key) = self.admin_key {
                        self.socket.send(
                            serde_json::to_string(&ClientCmd::AdminChallenge(
                                self.client_id,
                                admin_key,
                            ))
                            .map_err(Into::into),
                        );
                    }
                }
            }
            Msg::SocketRecv(text) => {
                let push: ServerPush = serde_json::from_str(&text).unwrap(); //FIXME
                debug!("ws recv={:?}", push);

                return match push {
                    ServerPush::StateChange { new_state } => {
                        let players_diff = self.players.neq_assign(new_state.players.clone());
                        let calling_diff = self.is_calling.neq_assign(new_state.is_calling);
                        players_diff || calling_diff
                    }
                    ServerPush::IsAdminUser => self.is_admin.neq_assign(true),
                };
            }
            Msg::SelectCard(idx) => {
                self.socket.send(
                    serde_json::to_string(&ClientCmd::SetPlayerCard(self.client_id, Some(idx)))
                        .map_err(Into::into),
                );
            }
            Msg::SetPlayerName(name) => {
                self.socket.send(
                    serde_json::to_string(&ClientCmd::SetPlayerName(self.client_id, name))
                        .map_err(Into::into),
                );
            }
            Msg::ToggleCalling => {
                let cmd = if self.is_calling {
                    ClientCmd::Resume
                } else {
                    ClientCmd::Call
                };
                self.socket
                    .send(serde_json::to_string(&cmd).map_err(Into::into))
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if !self.players.contains_key(&self.client_id) {
            return html! {};
        }

        let player_name = self.get_player_name().unwrap();

        html! {
        <div class="container mx-auto flex flex-col space-y-4">
            <PlayerCards players=self.players.clone() is_calling=self.is_calling />
            <div>
            <label for="player-name">{"Name:"}</label>
            <TextEdit
                id="player-name"
                value=player_name
                onsubmit=self.link.callback(Msg::SetPlayerName)/>
            </div>
            {self.build_card_picker()}
            {self.build_call_button()}
        </div>
        }
    }
}

impl App {
    fn get_player_name(&self) -> Option<String> {
        self.players
            .get(&self.client_id)
            .as_ref()
            .map(|p| p.name.clone())
    }
    fn build_call_button(&self) -> Html {
        if !self.is_admin {
            return html! {};
        }
        let on_click = self.link.callback(|_| Msg::ToggleCalling);
        html! { <button class="btn-red" onclick=on_click>{"Call"}</button> }
    }
    fn build_card_picker(&self) -> Html {
        let player = self.players.get(&self.client_id);
        if player.is_none() {
            return html! {};
        }
        let player = player.unwrap();

        let mut classes = vec![
            "card-picker",
            "grid",
            "grid-flow-row",
            "grid-cols-4",
            "sm:grid-cols-6",
            "md:grid-cols-12",
            "gap-8",
            "py-4",
        ];
        if self.is_calling {
            classes.push("calling");
        }
        html! {
        <div>
        <p>{format!("{}, please select a card:", player.name)}</p>

        <ul class=classes>
            {for CARDS.iter().enumerate()
                .map(|(idx, name)| {
                    let on_click = if self.is_calling  {
                        Callback::noop()
                    } else {
                        self.link.callback(move |_| Msg::SelectCard(idx))
                    };
                    let is_active = if player.selected_card == Some(idx) { "active" } else { "" };
                    html!{ <li key=*name class=("card", is_active) onclick=on_click>
                    <div class="value">{name}</div>
                    </li> }

                })
            }
        </ul>
        </div>
        }
    }
}
