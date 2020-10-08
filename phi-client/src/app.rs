use crate::agents::store::{PlanningPokerStore, Player, PlayerId, Request};
use crate::player_cards::PlayerCards;
use crate::text_edit::TextEdit;
use std::collections::HashMap;
use yew::callback::Callback;
use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
use yewtil::NeqAssign;

pub struct App {
    link: ComponentLink<Self>,
    client_id: PlayerId,
    players: HashMap<PlayerId, Player>,
    is_calling: bool,
    store: Box<dyn Bridge<StoreWrapper<PlanningPokerStore>>>,
}

#[derive(Debug)]
pub enum Msg {
    SelectCard(usize),
    SetPlayerName(String),
    StoreChange(ReadOnly<PlanningPokerStore>),
    ToggleCalling,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut store = PlanningPokerStore::bridge(link.callback(Msg::StoreChange));
        let client_id = Player::next_id();
        store.send(Request::InitAdminClient(client_id));
        store.send(Request::AddPlayer(client_id, String::from("Guest")));

        // FIXME: these players are stand-ins. Remove once we have a server.
        store.send(Request::AddPlayer(Player::next_id(), String::from("Alice")));
        store.send(Request::AddPlayer(Player::next_id(), String::from("Bob")));

        App {
            link,
            client_id,
            store,
            players: Default::default(),
            is_calling: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectCard(idx) => {
                self.store
                    .send(Request::ChangePlayerCard(self.client_id, Some(idx)));
            }
            Msg::SetPlayerName(name) => {
                self.store.send(Request::RenamePlayer(self.client_id, name));
            }
            Msg::StoreChange(store) => {
                let store = store.borrow();
                let players_diff = self.players.neq_assign(store.players.clone());
                let calling_diff = self.is_calling.neq_assign(store.is_calling);
                return players_diff || calling_diff;
            }
            Msg::ToggleCalling => {
                self.store.send(if self.is_calling {
                    Request::Resume
                } else {
                    Request::Call
                });
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
        // FIXME: only admin type users should get the button
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
            {for PlanningPokerStore::CARDS.iter().enumerate()
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
