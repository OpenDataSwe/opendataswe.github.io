use crate::info::Info;
use crate::input::selected_ind;
use skatt_lib::generated::municipalities::Municipalities;
use web_sys::Event;
use yew::{html, Callback, Component, Context, Html, Properties};

pub struct MunicipalityPicker;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct MunicipalityProps {
    pub sel: Option<Municipalities>,
    pub on_change: Callback<Result<Option<Municipalities>, String>>,
    pub info_html: &'static str,
}

fn municipality_value(input_event: &Event) -> Result<Option<Municipalities>, String> {
    let ind = selected_ind(input_event)?;
    Ok(Municipalities::VALUES.get(ind - 1).copied())
}

impl Component for MunicipalityPicker {
    type Message = ();
    type Properties = MunicipalityProps;

    #[inline]
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let m = &Municipalities::VALUES;
        let MunicipalityProps {
            sel,
            on_change,
            info_html,
        } = ctx.props().clone();
        let onchange = Callback::from(move |input: Event| {
            on_change.emit(municipality_value(&input));
        });
        html! {
            <div>
                <label for="municipality">{"Kommun "}</label><Info content={info_html}/>
                <br/>
                <select name="municipality" id="municipality" {onchange}>
                    <option selected={sel.is_none()}></option>
                    {
                        m.iter().map(|m| {
                            if sel.as_ref() == Some(m) {
                                html! {
                                    <option value={m.display_name()} selected=true >{m.display_name()}</option>
                                }
                            } else {
                                html! {
                                    <option value={m.display_name()}>{m.display_name()}</option>
                                }
                            }

                        }).collect::<Html>()

                    }
                </select>
            </div>
        }
    }
}
