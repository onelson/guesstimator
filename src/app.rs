use crate::agents::store::{PlanningPokerStore, Player, PlayerId, Request};
use crate::player_cards::PlayerCards;
use crate::text_edit::TextEdit;
use std::collections::HashMap;
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
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let player = self.players.get(&self.client_id);
        if player.is_none() {
            return html! {};
        }
        let player_name = &player.as_ref().unwrap().name;
        html! {
        <div class="container mx-auto">
            <PlayerCards players=self.players.clone() is_calling=self.is_calling />
            <div>
            <label for="player-name">{"Name:"}</label>
            <TextEdit
                id="player-name"
                value=player_name
                onsubmit=self.link.callback(Msg::SetPlayerName)/>
            </div>
            <p>{format!("{}, please select a card:", player_name)}</p>
            {self.build_card_picker()}
        </div>
        }
    }
}

impl App {
    fn build_card_picker(&self) -> Html {
        let player = self.players.get(&self.client_id);
        if player.is_none() {
            return html! {};
        }

        let player = player.unwrap();
        html! {
        <ul class="flex flex-row space-x-4 pt-8">
            {for PlanningPokerStore::CARDS.iter().enumerate()
                .map(|(idx, name)| {
                    let on_click = self.link.callback(move |_| Msg::SelectCard(idx));
                    let is_active = if player.selected_card == Some(idx) { "active" } else { "" };
                    html!{ <li key=*name class=("card", is_active) onclick=on_click>{name}</li> }
                })
            }
        </ul>
        }
    }
}
