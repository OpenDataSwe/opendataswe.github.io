use crate::check::Checkbox;
use crate::error::SimpleErrorDisplay;
use crate::info::Info;
use crate::input::{valid_year, NumberInput};
use crate::plot::picker::PlotPicker;
use crate::select::church::ChurchSelect;
use crate::select::municipality::MunicipalityPicker;
use rustc_hash::FxHashMap;
use skatt_lib::api::base_deduction::BaseDeduction;
use skatt_lib::api::tax_by_municipality::MunicipalTaxSpecification;
use skatt_lib::api::Client;
use skatt_lib::calculate::under_66_years_old;
use skatt_lib::generated::municipalities::Municipalities;
use skatt_lib::generated::readme::{ALDER, KOMMUN, MEDLEM_I_SVENSKA_KYRKAN, SKATTEBERAKNING};
use skatt_lib::CURRENT_YEAR;
use std::rc::Rc;
use yew::virtual_dom::VNode;
use yew::{html, Component, Context, Html};

pub enum NoneFetchOrSet<T> {
    None,
    Fetching,
    Set(T),
}

impl<T> NoneFetchOrSet<T> {
    fn get(&self) -> Option<&T> {
        match self {
            NoneFetchOrSet::None | NoneFetchOrSet::Fetching => None,
            NoneFetchOrSet::Set(v) => Some(v),
        }
    }
}

pub struct App {
    skv_client: Client,
    birth_year: Option<u16>,
    base_deductions_under_66: NoneFetchOrSet<Rc<Vec<BaseDeduction>>>,
    base_deductions_66_and_over: NoneFetchOrSet<Rc<Vec<BaseDeduction>>>,
    err: Option<String>,
    municipality: Option<Municipalities>,
    municipality_data: NoneFetchOrSet<FxHashMap<String, MunicipalTaxSpecification>>,
    fetching_municipality_data: bool,
    church_member: bool,
    selected_congregation: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            skv_client: Client::new(),
            birth_year: None,
            base_deductions_under_66: NoneFetchOrSet::None,
            err: None,
            municipality: None,
            municipality_data: NoneFetchOrSet::None,
            fetching_municipality_data: false,
            church_member: false,
            selected_congregation: None,
            base_deductions_66_and_over: NoneFetchOrSet::None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct BaseCalculationData {
    pub birth_year: u16,
    pub church_member: bool,
    pub municipal_tax_specification: MunicipalTaxSpecification,
    pub base_deductions: Rc<Vec<BaseDeduction>>,
}

pub enum AppMessage {
    BirthYearInput(Result<Option<i32>, String>),
    BaseDeductionsFetch(u16, Result<Vec<BaseDeduction>, String>),
    MunicipalityInput(Result<Option<Municipalities>, String>),
    FetchedMunicipalityData(Result<FxHashMap<String, MunicipalTaxSpecification>, String>),
    ChurchMemberInput(Result<bool, String>),
    SelectCongregation(Result<Option<String>, String>),
}

impl App {
    fn display_plot(&self) -> Option<VNode> {
        match (
            &self.base_deductions_under_66,
            &self.base_deductions_66_and_over,
            self.birth_year,
            &self.municipality_data,
            self.selected_congregation.as_ref(),
        ) {
            (
                bd_under,
                bd_over,
                Some(birth_year),
                NoneFetchOrSet::Set(municipality_data),
                congregation,
            ) => {
                if valid_year(birth_year) {
                    let tax_spec = if let Some(congregation) = congregation {
                        municipality_data
                            .get(congregation)
                            .ok_or_else(|| format!("Missing congregation={congregation} (API err)"))
                    } else {
                        municipality_data
                            .values()
                            .next()
                            .ok_or_else(|| "No congregations found (API err)".to_string())
                    };
                    let tax_spec = match tax_spec {
                        Ok(t) => t.clone(),
                        Err(e) => {
                            return Some(html! {
                              <SimpleErrorDisplay message={Some(e)}/>
                            })
                        }
                    };
                    let use_bd = if under_66_years_old(birth_year) {
                        bd_under.get().map(std::clone::Clone::clone)
                    } else {
                        bd_over.get().map(std::clone::Clone::clone)
                    };
                    use_bd.map(|bd| {
                        let base = BaseCalculationData {
                            birth_year,
                            church_member: self.church_member,
                            municipal_tax_specification: tax_spec,
                            base_deductions: bd.clone(),
                        };
                        html! {
                            <div>
                                <PlotPicker base_calculation_data={base} />
                            </div>
                        }
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    #[expect(clippy::too_many_lines)]
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::BirthYearInput(y) => match y {
                Ok(y) => {
                    let y = y.and_then(|y| u16::try_from(y).ok());
                    self.birth_year = y;
                    if let Some(y) = self.birth_year {
                        // This nesting...
                        if valid_year(y) {
                            if under_66_years_old(y) {
                                if matches!(self.base_deductions_under_66, NoneFetchOrSet::None) {
                                    self.base_deductions_under_66 = NoneFetchOrSet::Fetching;
                                    let cl = self.skv_client.clone();
                                    ctx.link().send_future(async move {
                                        let res_under = cl
                                            .fetch_base_deductions(CURRENT_YEAR, true)
                                            .await
                                            .map_err(|_e| {
                                                "Failed to fetch base deductions".to_string()
                                            });
                                        Self::Message::BaseDeductionsFetch(y, res_under)
                                    });
                                }
                            } else if matches!(
                                self.base_deductions_66_and_over,
                                NoneFetchOrSet::None
                            ) {
                                self.base_deductions_66_and_over = NoneFetchOrSet::Fetching;
                                let cl = self.skv_client.clone();
                                ctx.link().send_future(async move {
                                    let res_under = cl
                                        .fetch_base_deductions(CURRENT_YEAR, false)
                                        .await
                                        .map_err(|_e| {
                                            "Failed to fetch base deductions".to_string()
                                        });
                                    Self::Message::BaseDeductionsFetch(y, res_under)
                                });
                            }
                        }
                    }
                    self.err = None;
                }
                Err(e) => {
                    self.err = Some(e);
                }
            },
            AppMessage::BaseDeductionsFetch(year, bds) => match bds {
                Ok(b) => {
                    if under_66_years_old(year) {
                        self.base_deductions_under_66 = NoneFetchOrSet::Set(Rc::new(b));
                    } else {
                        self.base_deductions_66_and_over = NoneFetchOrSet::Set(Rc::new(b));
                    }
                }
                Err(e) => {
                    if under_66_years_old(year) {
                        self.base_deductions_under_66 = NoneFetchOrSet::None;
                    } else {
                        self.base_deductions_66_and_over = NoneFetchOrSet::None;
                    }
                    self.err = Some(e);
                }
            },
            AppMessage::MunicipalityInput(m) => match m {
                Ok(Some(m)) => {
                    let cl = self.skv_client.clone();
                    self.municipality = Some(m);
                    self.err = None;
                    self.fetching_municipality_data = true;
                    ctx.link().send_future(async move {
                        let cg = cl
                            .fetch_tax_for_municipality_by_congregation(m, CURRENT_YEAR)
                            .await
                            .map_err(|_e| format!("Failed to fetch data for {m}"));
                        Self::Message::FetchedMunicipalityData(cg)
                    });
                }
                Ok(None) => {
                    self.municipality = None;
                    self.err = None;
                }
                Err(e) => {
                    self.err = Some(e);
                }
            },
            AppMessage::FetchedMunicipalityData(d) => {
                self.fetching_municipality_data = false;
                match d {
                    Ok(d) => {
                        self.municipality_data = NoneFetchOrSet::Set(d);
                        self.err = None;
                    }
                    Err(e) => {
                        self.err = Some(e);
                    }
                }
            }
            AppMessage::ChurchMemberInput(r) => match r {
                Ok(member) => {
                    self.church_member = member;
                    self.err = None;
                }
                Err(e) => {
                    self.err = Some(e);
                }
            },
            AppMessage::SelectCongregation(r) => match r {
                Ok(s) => {
                    self.selected_congregation = s;
                    self.err = None;
                }
                Err(e) => {
                    self.err = Some(e);
                }
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Self::Message::BirthYearInput);
        let v = if let Some(y) = self.birth_year {
            y.to_string()
        } else {
            String::new()
        };
        let when_loading_year = if matches!(self.base_deductions_under_66, NoneFetchOrSet::Fetching)
        {
            Some(html! {
                <strong>{"Laddar grundavdrag under 66..."}</strong>
            })
        } else if matches!(self.base_deductions_66_and_over, NoneFetchOrSet::Fetching) {
            Some(html! {
                <strong>{"Laddar grundavdrag 66 och äldre..."}</strong>
            })
        } else {
            None
        };
        let on_municipality_change = ctx.link().callback(Self::Message::MunicipalityInput);
        let when_loading_municipality_data = self.fetching_municipality_data.then(|| {
            html! {
                <strong>{"Laddar kommundata..."}</strong>
            }
        });
        let on_church_member_change = ctx.link().callback(Self::Message::ChurchMemberInput);
        let select_congregation = if let Some(cg) = self.municipality_data.get() {
            let mut opts = cg.keys().cloned().collect::<Vec<_>>();
            opts.sort();
            let on_change = ctx.link().callback(Self::Message::SelectCongregation);
            if self.church_member {
                Some(html! {
                    <ChurchSelect opts={opts} sel={self.selected_congregation.clone()} on_change={on_change}/>
                })
            } else {
                None
            }
        } else {
            None
        };
        let plot = self.display_plot();
        let n: Option<String> = None;

        html! {
            <>
                <div>
                    <span>{"Koden för uträkningarna finns "}</span><a href="https://github.com/OpenDataSwe/opendataswe.github.io">{"här."}</a><span>{" "}</span><Info content={SKATTEBERAKNING}/>
                </div>
                <br/>
                <div>
                    <NumberInput input_id={"dob"} input_name={"dob"} label_text={"Födelseår "} err_msg={n} {on_change} info_html={ALDER} value={v} />
                    {when_loading_year}
                </div>
                <br/>
                <div>
                    <MunicipalityPicker sel={self.municipality} on_change={on_municipality_change} info_html={KOMMUN} />
                    {when_loading_municipality_data}
                </div>
                <SimpleErrorDisplay message={self.err.clone()}/>
                <br/>
                <div>
                    <Checkbox info_html={MEDLEM_I_SVENSKA_KYRKAN} on_change={on_church_member_change} checked={self.church_member} label_text={"Medlem i Svenska kyrkan "} input_name={"church_member"} input_id={"church_member"} />
                </div>
                {
                    select_congregation
                }
                <br/>
                {plot}
            </>

        }
    }
}
