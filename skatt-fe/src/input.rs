use crate::error::SimpleErrorDisplay;
use crate::info::Info;
use skatt_lib::CURRENT_YEAR;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{html, Callback, Component, Context, Event, Html, InputEvent, Properties};

pub fn input_target(e: InputEvent) -> Result<HtmlInputElement, String> {
    let event: Event = e
        .dyn_into()
        .map_err(|_e| "Failed to convert InputEvent to Event")?;
    let event_target = event
        .target()
        .ok_or("Failed to get event target for input")?;
    let target: HtmlInputElement = event_target
        .dyn_into()
        .map_err(|_| "Event target was not a html input element".to_string())?;
    Ok(target)
}

pub fn text_value_from_input_event(e: InputEvent) -> Result<Option<String>, String> {
    let target = input_target(e)?;
    let val = target.value();
    if val.is_empty() {
        Ok(None)
    } else {
        Ok(Some(val))
    }
}

pub fn num_value_from_input_event(e: InputEvent) -> Result<Option<i32>, String> {
    text_value_from_input_event(e)?
        .map(|s| {
            s.parse()
                .map_err(|_e| "Invalid number from input event".to_string())
        })
        .transpose()
}

pub fn bool_value_from_input_event(e: InputEvent) -> Result<bool, String> {
    let target = input_target(e)?;
    Ok(target.checked())
}

pub fn selected_ind(e: &Event) -> Result<usize, String> {
    let event_target = e.target().ok_or("Failed to get event target for input")?;
    let target: HtmlSelectElement = event_target
        .dyn_into()
        .map_err(|_| "Event target was not a html select element".to_string())?;
    let sel = target.selected_index();
    usize::try_from(sel).map_err(|_| format!("Selected index={sel} was not a usize"))
}

#[must_use]
pub fn valid_year(y: u16) -> bool {
    (1885..=CURRENT_YEAR).contains(&y)
}

pub struct NumberInput;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NumberInputProps {
    pub label_text: &'static str,
    pub input_id: &'static str,
    pub input_name: &'static str,
    pub err_msg: Option<String>,
    pub value: Option<String>,
    pub on_change: Callback<Result<Option<i32>, String>>,
    pub info_html: &'static str,
}

impl Component for NumberInput {
    type Message = ();
    type Properties = NumberInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let NumberInputProps {
            label_text,
            input_id,
            input_name,
            err_msg,
            value,
            on_change,
            info_html,
        } = ctx.props().clone();
        let oninput = Callback::from(move |input: InputEvent| {
            on_change.emit(num_value_from_input_event(input));
        });
        let value = value.unwrap_or_default();
        html! {
            <div>
                <label for={input_name}>{label_text}</label><Info content={info_html}/>
                <br/>
                <SimpleErrorDisplay message={err_msg}/>
                <input type="text" name={input_name} id={input_id} {value} {oninput}/>
            </div>
        }
    }
}
