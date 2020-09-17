use log::*;
use serde_derive::{Deserialize, Serialize};
// use yew::format::Json;
use crate::text_edit::TextEdit;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "guesstimation";
const CARDS: [&str; 12] = [
    "0", "1", "2", "3", "5", "8", "13", "21", "100", "∞", "?", "☕",
];

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    selected_card: Option<usize>,
    player_name: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Msg {
    SelectCard(usize),
    SetPlayerName(String),
    Noop,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state = State::default();
        App {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        debug!("{:?}", &msg);
        match msg {
            Msg::SelectCard(idx) => {
                match self.state.selected_card.take() {
                    Some(prev) if prev == idx => (),
                    _ => self.state.selected_card = Some(idx),
                }

                return true;
            }
            Msg::SetPlayerName(name) => {
                self.state.player_name = Some(name);
                return true;
            }
            Msg::Noop => (),
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let player_name = self
            .state
            .player_name
            .as_ref()
            .map(|x| x.as_str())
            .unwrap_or_else(|| "guest");

        html! {
        <div class="container mx-auto">
            <div>
            <label for="player-name">{"Name:"}</label>
            <TextEdit
                id="player-name"
                value=player_name
                onsubmit=self.link.callback(Msg::SetPlayerName)/>
            </div>
            <p>{format!("{}, please select a card:", &player_name)}</p>
            <ul class="flex flex-row space-x-4 pt-8">
            {for CARDS.iter().enumerate()
                .map(|(idx, name)| {
                    let on_click = self.link.callback(move |_| Msg::SelectCard(idx));
                    let is_active = if self.state.selected_card == Some(idx) { "active" } else { "" };
                    html!{ <li key=*name class=("card", is_active) onclick=on_click>{name}</li> }
                })}
            </ul>
        </div>
        }
    }
}
