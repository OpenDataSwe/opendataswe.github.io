use yew::{html, Component, Context, Html, Properties};

pub struct SimpleErrorDisplay;

#[derive(Properties, PartialEq, Clone)]
pub struct SimpleErrorDisplayProps {
    pub message: Option<String>,
}

impl Component for SimpleErrorDisplay {
    type Message = ();
    type Properties = SimpleErrorDisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { ctx.props().message.clone().map(|m| html!{ <strong>{m}</strong>})}
            </>
        }
    }
}
