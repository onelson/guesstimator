//! The "Player Cards" component represents the selections made by all players
//! in the game.
//!
//! One card is shown for each player, and the front or back of the cards are
//! displayed depending on whether or not the admin of the game session is
//! "calling" or not.
//!

use crate::agents::store::{Player, PlayerId};
use std::collections::HashMap;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct PlayerCards {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub players: HashMap<PlayerId, Player>,
    pub is_calling: bool,
}

impl Component for PlayerCards {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PlayerCards { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
        <>
            <h1>{"Poker!"}</h1>
            <div class="flex space-4">
            {for self.props.players.iter().map(|(k, v)|
                html!{ <div key=k.to_string() class=(
                "card", if v.selected_card.is_none() { "undecided" } else { "" }
                )>{&v.name}</div>})
            }
            </div>
        </>
        }
    }
}
