use crate::info::Info;
use crate::input::bool_value_from_input_event;
use web_sys::InputEvent;
use yew::{html, Callback, Component, Context, Html, Properties};

pub struct Checkbox;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct CheckboxProps {
    pub label_text: &'static str,
    pub input_name: &'static str,
    pub input_id: &'static str,
    pub checked: bool,
    pub on_change: Callback<Result<bool, String>>,
    pub info_html: &'static str,
}

impl Component for Checkbox {
    type Message = ();
    type Properties = CheckboxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CheckboxProps {
            label_text,
            input_name,
            input_id,
            checked,
            on_change,
            info_html,
        } = ctx.props().clone();
        let oninput = ctx.link().callback(move |evt: InputEvent| {
            on_change.emit(bool_value_from_input_event(evt));
        });
        html! {
            <div>
                <label for={input_name}>{label_text}</label><Info content={info_html}/>
                <input name={input_name} id={input_id} type="checkbox" {oninput} checked={checked}/>
            </div>
        }
    }
}
