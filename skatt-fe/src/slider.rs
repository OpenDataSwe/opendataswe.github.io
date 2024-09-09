use crate::input::num_value_from_input_event;
use web_sys::InputEvent;
use yew::{html, Callback, Component, Context, Html, Properties};

pub struct Slider;

#[derive(Properties, PartialEq, Clone)]
pub struct SliderProps {
    pub value: i32,
    pub on_change: Callback<Result<Option<i32>, String>>,
}

impl Component for Slider {
    type Message = ();
    type Properties = SliderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let SliderProps { value, on_change } = ctx.props().clone();
        let oninput = ctx
            .link()
            .callback(move |e: InputEvent| on_change.emit(num_value_from_input_event(e)));

        html! {
            <div>
                <label for="slider">{"Lön"}</label><input type="number" value={value.to_string()} oninput={oninput.clone()}/>
                <input type="range" min="100" max={"1000000"} step="100" value={value.to_string()} id="slider" name="slider" {oninput} />
            </div>
        }
    }
}
