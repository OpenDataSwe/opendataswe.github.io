use crate::input::selected_ind;
use web_sys::Event;
use yew::{html, Callback, Component, Context, Html, Properties};

pub struct ChurchSelect;

#[derive(Properties, PartialEq, Clone)]
pub struct ChurchSelectProps {
    pub sel: Option<String>,
    pub opts: Vec<String>,
    pub on_change: Callback<Result<Option<String>, String>>,
}

fn church_from_sel(event: &Event, opts: &[String]) -> Result<Option<String>, String> {
    let ind = selected_ind(event)?;
    let sel = opts.get(ind - 1);
    if let Some(sel) = sel {
        if sel.is_empty() {
            Ok(None)
        } else {
            Ok(Some(sel.clone()))
        }
    } else {
        Ok(None)
    }
}

impl Component for ChurchSelect {
    type Message = ();
    type Properties = ChurchSelectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ChurchSelectProps {
            opts,
            sel,
            on_change,
        } = ctx.props().clone();
        let opts_c = opts.clone();
        let onchange = Callback::from(move |input: Event| {
            on_change.emit(church_from_sel(&input, &opts_c));
        });
        html! {
            <>
                <label for="churh_name_sel">{"Församling"}</label>
                <br/>
                <select name="church_name_sel" id="church_name_sel" {onchange}>
                    <option selected={sel.is_none()}></option>
                    {
                        opts.iter().map(|c| {
                            if sel.as_ref() == Some(c) {
                                html! {
                                    <option value={c.clone()} selected=true >{c}</option>
                                }
                            } else {
                                html! {
                                    <option value={c.clone()}>{c}</option>
                                }
                            }

                        }).collect::<Html>()

                    }
                </select>
            </>
        }
    }
}
