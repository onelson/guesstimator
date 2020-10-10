//! The "Player Cards" component represents the selections made by all players
//! in the game.
//!
//! One card is shown for each player, and the front or back of the cards are
//! displayed depending on whether or not the admin of the game session is
//! "calling" or not.
//!

use phi_common::{Player, PlayerId};
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
        let mut classes = vec!["player-cards", "flex", "space-x-2", "py-4"];

        if self.props.is_calling {
            classes.push("calling");
        }

        html! {
        <>
            <div class=classes>
            {for self.props.players.iter().map(|(k, v)|
                html!{
                <div key=k.to_string()>
                    <div class=(
                        "card", if v.selected_card.is_none() { "undecided" } else { "" }
                    )>
                        <div class="value">{v.selected_card_name().unwrap_or("")}</div>
                    </div>
                    <div class="name text-center">{&v.name}</div>
                </div>
                })
            }
            </div>
        </>
        }
    }
}
