//! Almost verbatim from
//! <https://github.com/yewstack/yew/blob/master/examples/store/src/text_input.rs>

use yew::prelude::*;

pub struct TextEdit {
    link: ComponentLink<Self>,
    text: String,
    props: TextEditProperties,
}

pub enum TextEditMsg {
    SetText(String),
    Submit,
    None,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TextEditProperties {
    #[prop_or_default]
    pub id: String,
    pub value: String,
    pub onsubmit: Callback<String>,
}

impl Component for TextEdit {
    type Message = TextEditMsg;
    type Properties = TextEditProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextEdit {
            link,
            text: props.value.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextEditMsg::SetText(text) => self.text = text,
            TextEditMsg::Submit => {
                let text = std::mem::replace(&mut self.text, self.props.value.clone());
                self.props.onsubmit.emit(text);
            }
            TextEditMsg::None => return false,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.text = self.props.value.clone();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="text"
                id=self.props.id
                value=&self.text
                oninput=self.link.callback(|e: InputData| TextEditMsg::SetText(e.value))
                onkeydown=self.link.callback(move |e: KeyboardEvent| {
                    e.stop_propagation();
                    if e.key() == "Enter" { TextEditMsg::Submit } else { TextEditMsg::None }
                })
            />
        }
    }
}
