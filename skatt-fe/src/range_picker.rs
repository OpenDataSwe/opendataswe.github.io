use crate::input::NumberInput;
use skatt_lib::calculate::CalculationRange;
use skatt_lib::generated::readme::{SLUT, START, STEG};
use skatt_lib::number::MaxNumberValue;
use yew::{html, Callback, Component, Context, Html, Properties};

pub struct RangePicker {
    start_input: Option<MaxNumberValue>,
    step_input: Option<MaxNumberValue>,
    end_input: Option<MaxNumberValue>,
    start_input_err: Option<String>,
    step_input_err: Option<String>,
    end_input_err: Option<String>,
}

impl RangePicker {
    fn has_valid_range(&self) -> Option<CalculationRange> {
        if let (Some(start), Some(step), Some(end)) =
            (self.start_input, self.step_input, self.end_input)
        {
            let res_end = start.into_i32().checked_add(step.into_i32())?;
            if res_end < end.into_i32() && step.into_i32() > 0 {
                return Some(CalculationRange { start, step, end });
            }
        }
        None
    }

    fn update_start_changed(&mut self, start: Result<Option<i32>, String>) {
        match start {
            Ok(v) => {
                let v = v
                    .and_then(|val| MaxNumberValue::try_new(val).ok())
                    .inspect(|&start| {
                        if let Some(end) = self.end_input {
                            if start > end {
                                self.step_input_err = Some("Start larger than end".to_string());
                            }
                            if let Some(step) = self.step_input {
                                match start.into_i32().checked_add(step.into_i32()) {
                                    None => {
                                        self.step_input_err =
                                            Some("Start + step overflow".to_string());
                                    }
                                    Some(res) => {
                                        if res > end.into_i32() {
                                            self.step_input_err =
                                                Some("Start + step larger than end".to_string());
                                        }
                                    }
                                };
                            }
                        }
                    });
                self.start_input = v;
            }
            Err(e) => {
                self.start_input_err = Some(e);
            }
        }
    }

    fn update_step_changed(&mut self, step: Result<Option<i32>, String>) {
        match step {
            Ok(s) => {
                let use_val = s
                    .and_then(|val| MaxNumberValue::try_new(val).ok())
                    .inspect(|&v| {
                        if let Some(end) = self.end_input {
                            if end < v {
                                self.step_input_err = Some("Step larger than end".to_string());
                            }
                            if let Some(start) = self.start_input {
                                match start.into_i32().checked_add(v.into_i32()) {
                                    None => {
                                        self.step_input_err =
                                            Some("Step + start overflow".to_string());
                                    }
                                    Some(res) => {
                                        if res > end.into_i32() {
                                            self.step_input_err =
                                                Some("Step + start larger than end".to_string());
                                        }
                                    }
                                }
                            }
                            if v == MaxNumberValue::ZERO {
                                self.step_input_err =
                                    Some("Step must be larger than 0".to_string());
                            }
                        }
                    });
                self.step_input = use_val;
            }
            Err(e) => {
                self.step_input_err = Some(e);
            }
        }
    }

    fn update_end_changed(&mut self, end: Result<Option<i32>, String>) {
        match end {
            Ok(s) => {
                let use_val = s
                    .and_then(|val| MaxNumberValue::try_new(val).ok())
                    .inspect(|&v| {
                        if let Some(start) = self.start_input {
                            if start > v {
                                self.end_input_err = Some("End smaller than start".to_string());
                            }
                            if let Some(step) = self.step_input {
                                match start.into_i32().checked_add(step.into_i32()) {
                                    None => {
                                        self.end_input_err =
                                            Some("Step + start overflow".to_string());
                                    }
                                    Some(end) => {
                                        if end > v.into_i32() {
                                            self.end_input_err =
                                                Some("Step + start larger than end".to_string());
                                        }
                                    }
                                }
                            }
                        }
                    });
                self.end_input = use_val;
            }
            Err(e) => {
                self.end_input_err = Some(e);
            }
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RangePickerProps {
    pub(crate) initial: CalculationRange,
    pub on_change: Callback<CalculationRange>,
}

pub enum RangePickerMessage {
    StartChanged(Result<Option<i32>, String>),
    StepChanged(Result<Option<i32>, String>),
    EndChanged(Result<Option<i32>, String>),
}

impl Component for RangePicker {
    type Message = RangePickerMessage;
    type Properties = RangePickerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            start_input: Some(ctx.props().initial.start),
            step_input: Some(ctx.props().initial.step),
            end_input: Some(ctx.props().initial.end),
            start_input_err: None,
            step_input_err: None,
            end_input_err: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // On any valid input (number is valid) the user should be
            // allowed to continue (value is accepted), even though it might not be structurally valid
            // as part of a range
            RangePickerMessage::StartChanged(start) => self.update_start_changed(start),
            RangePickerMessage::StepChanged(step) => self.update_step_changed(step),
            RangePickerMessage::EndChanged(end) => self.update_end_changed(end),
        }
        if let Some(range) = self.has_valid_range() {
            self.start_input_err = None;
            self.step_input_err = None;
            self.end_input_err = None;
            ctx.props().on_change.emit(range);
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_start_changed = ctx.link().callback(Self::Message::StartChanged);
        let on_step_changed = ctx.link().callback(Self::Message::StepChanged);
        let on_end_changed = ctx.link().callback(Self::Message::EndChanged);
        html! {
            <>
                <NumberInput info_html={START} input_id={"start_calc"} input_name={"start_calc"} label_text={"Start "} err_msg={self.start_input_err.clone()} on_change={on_start_changed} value={self.start_input.map(|s| s.to_string()).unwrap_or_default()} />
                <NumberInput info_html={STEG} input_id={"step_calc"} input_name={"step_calc"} label_text={"Steg "} err_msg={self.step_input_err.clone()} on_change={on_step_changed} value={self.step_input.map(|s| s.to_string()).unwrap_or_default()} />
                <NumberInput info_html={SLUT} input_id={"end_calc"} input_name={"end_calc"} label_text={"Slut "} err_msg={self.end_input_err.clone()} on_change={on_end_changed} value={self.end_input.map(|s| s.to_string()).unwrap_or_default()} />
            </>

        }
    }
}
