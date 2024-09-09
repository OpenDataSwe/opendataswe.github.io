use crate::app::BaseCalculationData;
use crate::error::SimpleErrorDisplay;
use crate::info::Info;
use crate::input::selected_ind;
use crate::plot::line_absolute::LineAbsolutePlot;
use crate::plot::line_absolute_incline::LineAbsoluteInclinePlot;
use crate::plot::line_percentage::LinePercentagePlot;
use crate::plot::line_percentage_incline::LinePercentageInclinePlot;
use crate::plot::pie::PiePlot;
use crate::plot::table::TaxTable;
use crate::range_picker::RangePicker;
use crate::slider::Slider;
use skatt_lib::calculate::CalculationRange;
use skatt_lib::generated::readme::{
    ABSOLUTGRAF, ABSOLUTGRAF_LUTNING, CIRKELDIAGRAM, PROCENTGRAF, PROCENTGRAF_LUTNING, TABELL,
};
use skatt_lib::number::MaxNumberValue;
use yew::virtual_dom::VNode;
use yew::{html, Component, Context, Html, Properties};

const MEDIAN_SALARY: MaxNumberValue = MaxNumberValue::const_new(35_600);

pub struct PlotPicker {
    salary: MaxNumberValue,
    err: Option<String>,
    calculation_range: CalculationRange,
    selected: PlotKind,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlotKind {
    LinePercentage,
    LinePercentageIncline,
    LineAbsolute,
    LineAbsoluteIncline,
    Pie,
    Table,
}

impl PlotKind {
    const fn help_html(self) -> &'static str {
        match self {
            PlotKind::LinePercentage => PROCENTGRAF,
            PlotKind::LinePercentageIncline => PROCENTGRAF_LUTNING,
            PlotKind::LineAbsolute => ABSOLUTGRAF,
            PlotKind::LineAbsoluteIncline => ABSOLUTGRAF_LUTNING,
            PlotKind::Pie => CIRKELDIAGRAM,
            PlotKind::Table => TABELL,
        }
    }
}

const LINE_PERC_TAG: &str = "Procentgraf";
const LINE_PERC_INC_TAG: &str = "Procentgraf lutning";
const LINE_ABS_TAG: &str = "Absolutgraf";
const LINE_ABS_INC_TAG: &str = "Absolutgraf lutning";
const PIE_TAG: &str = "Paj";
const TABLE_TAG: &str = "Tabell";

impl PlotKind {
    fn into_str(self) -> &'static str {
        match self {
            PlotKind::LinePercentage => LINE_PERC_TAG,
            PlotKind::LinePercentageIncline => LINE_PERC_INC_TAG,
            PlotKind::LineAbsolute => LINE_ABS_TAG,
            PlotKind::LineAbsoluteIncline => LINE_ABS_INC_TAG,
            PlotKind::Pie => PIE_TAG,
            PlotKind::Table => TABLE_TAG,
        }
    }
}

const OPTS: &[PlotKind] = &[
    PlotKind::LinePercentage,
    PlotKind::LinePercentageIncline,
    PlotKind::LineAbsolute,
    PlotKind::LineAbsoluteIncline,
    PlotKind::Pie,
    PlotKind::Table,
];

#[derive(Properties, PartialEq, Clone)]
pub struct PlotPickerProps {
    pub base_calculation_data: BaseCalculationData,
}

pub enum PlotPickerMessage {
    SelectionChanged(Result<usize, String>),
    SalaryChanged(Result<Option<i32>, String>),
    NewCalculationRange(CalculationRange),
}

impl PlotPicker {
    fn match_components(&self, ctx: &Context<Self>) -> (Option<VNode>, VNode) {
        let (slider, disp) = match self.selected {
            PlotKind::LinePercentage => {
                let on_change = ctx.link().callback(PlotPickerMessage::NewCalculationRange);
                (
                    None,
                    html! {
                        <>
                            <RangePicker initial={self.calculation_range} {on_change} />
                            <LinePercentagePlot base_calculation_data={ctx.props().base_calculation_data.clone()} calculation_range={self.calculation_range} />
                        </>
                    },
                )
            }
            PlotKind::LinePercentageIncline => {
                let on_change = ctx.link().callback(PlotPickerMessage::NewCalculationRange);
                (
                    None,
                    html! {
                        <>
                            <RangePicker initial={self.calculation_range} {on_change} />
                            <LinePercentageInclinePlot base_calculation_data={ctx.props().base_calculation_data.clone()} calculation_range={self.calculation_range} />
                        </>
                    },
                )
            }
            PlotKind::LineAbsolute => {
                let on_change = ctx.link().callback(PlotPickerMessage::NewCalculationRange);
                (
                    None,
                    html! {
                        <>
                            <RangePicker initial={self.calculation_range} {on_change} />
                            <LineAbsolutePlot base_calculation_data={ctx.props().base_calculation_data.clone()} calculation_range={self.calculation_range} />
                        </>
                    },
                )
            }
            PlotKind::LineAbsoluteIncline => {
                let on_change = ctx.link().callback(PlotPickerMessage::NewCalculationRange);
                (
                    None,
                    html! {
                        <>
                            <RangePicker initial={self.calculation_range} {on_change} />
                            <LineAbsoluteInclinePlot base_calculation_data={ctx.props().base_calculation_data.clone()} calculation_range={self.calculation_range} />
                        </>
                    },
                )
            }
            PlotKind::Pie => {
                let slider_on_change = ctx.link().callback(PlotPickerMessage::SalaryChanged);
                let slider = html! {
                    <div>
                        <Slider value={self.salary.into_i32()} on_change={slider_on_change}/>
                    </div>
                };
                (
                    Some(slider),
                    html! {
                        <PiePlot salary={self.salary} base_calculation_data={ctx.props().base_calculation_data.clone()} />
                    },
                )
            }
            PlotKind::Table => {
                let slider_on_change = ctx.link().callback(PlotPickerMessage::SalaryChanged);
                let slider = html! {
                    <div>
                        <Slider value={self.salary.into_i32()} on_change={slider_on_change}/>
                    </div>
                };
                (
                    Some(slider),
                    html! {
                        <TaxTable salary={self.salary} base_calculation_data={ctx.props().base_calculation_data.clone()} />
                    },
                )
            }
        };
        (slider, disp)
    }
}

const DEFAULT_RANGE: CalculationRange = CalculationRange {
    start: MaxNumberValue::ZERO,
    step: MaxNumberValue::const_new(100),
    end: MaxNumberValue::const_new(100_000),
};

impl Component for PlotPicker {
    type Message = PlotPickerMessage;
    type Properties = PlotPickerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            salary: MEDIAN_SALARY,
            err: None,
            calculation_range: DEFAULT_RANGE,
            selected: PlotKind::LinePercentage,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlotPickerMessage::SelectionChanged(sel) => match sel {
                Ok(s) => match s {
                    0 => self.selected = PlotKind::LinePercentage,
                    1 => self.selected = PlotKind::LinePercentageIncline,
                    2 => self.selected = PlotKind::LineAbsolute,
                    3 => self.selected = PlotKind::LineAbsoluteIncline,
                    4 => self.selected = PlotKind::Pie,
                    5 => self.selected = PlotKind::Table,
                    unk => self.err = Some(format!("Unknown plot selected={unk}")),
                },
                Err(e) => {
                    self.err = Some(e);
                }
            },
            PlotPickerMessage::SalaryChanged(ch) => match ch {
                Ok(v) => {
                    if let Some(v) = v {
                        match MaxNumberValue::try_new(v) {
                            Ok(val) => {
                                self.salary = val;
                            }
                            Err(e) => {
                                self.err = Some(e);
                            }
                        }
                    }
                }
                Err(e) => {
                    self.err = Some(e);
                }
            },
            PlotPickerMessage::NewCalculationRange(c) => {
                self.calculation_range = c;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_select = ctx
            .link()
            .callback(|e| Self::Message::SelectionChanged(selected_ind(&e)));
        let (slider, disp) = self.match_components(ctx);
        html! {
            <div>
                <div>
                    <label for="plot_picker">{"Visualisering "}</label>
                    <SimpleErrorDisplay message={self.err.clone()}/>
                    <select name="plot_picker" id="plot_picker" onchange={on_select}>
                    {
                        OPTS.iter()
                            .map(|o| {
                                if self.selected == *o {
                                    html! {
                                        <option value={o.into_str()} selected=true >{o.into_str()}</option>
                                    }
                                } else {
                                    html! {
                                        <option value={o.into_str()} >{o.into_str()}</option>
                                    }
                                }
                            })
                            .collect::<Html>()
                    }
                    </select>
                    <Info content={self.selected.help_html()}/>
                </div>
                {slider}
                <div>
                    {disp}
                </div>
            </div>
        }
    }
}
