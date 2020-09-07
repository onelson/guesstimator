use log::*;
use serde_derive::{Deserialize, Serialize};
// use yew::format::Json;
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

#[derive(Serialize, Deserialize)]
pub struct State;

#[derive(Debug, Eq, PartialEq)]
pub enum Msg {
    SelectCard(usize),
    Noop,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state = State;
        App {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        debug!("{:?}", &_msg);
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
        <>
            <ul>
            {for CARDS.iter().enumerate()
                .map(|(idx, name)| {
                    let on_click = self.link.callback(move |_| Msg::SelectCard(idx));
                    html!{<li onclick=on_click>{name}</li>}
                })}
            </ul>
        </>
        }
    }
}
