pub mod text;

use crate::info::text::RawHTML;
use web_sys::MouseEvent;
use yew::{html, Component, Context, Html, Properties};

pub struct Info {
    show: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct InfoProperties {
    pub content: &'static str,
}

pub enum InfoMessage {
    Close,
    Toggle,
}

impl Component for Info {
    type Message = InfoMessage;
    type Properties = InfoProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { show: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InfoMessage::Close => {
                self.show = false;
            }
            InfoMessage::Toggle => {
                self.show = !self.show;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_on_click = ctx.link().callback(|_evt: MouseEvent| Self::Message::Close);
        let toggle_on_click = ctx
            .link()
            .callback(|_evt: MouseEvent| Self::Message::Toggle);

        if self.show {
            html! {
                <>
                <button onclick={toggle_on_click} style="background:none;color:inherit">{"mindre"}</button>
                <div style="background-color:#e1e2e3;position:absolute;width:60%;margin-left:20%;padding-left:20px;padding-right:20px">
                    <span style="width:60%;z-index:1"><RawHTML inner_html={ctx.props().content}/></span>
                    <p onclick={close_on_click}>{"Stäng"}</p>
                </div>
                </>
            }
        } else {
            html! {
                <button onclick={toggle_on_click}>{"mer"}</button>
            }
        }
    }
}
