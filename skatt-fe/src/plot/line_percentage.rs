use crate::app::BaseCalculationData;
use crate::check::Checkbox;
use crate::error::SimpleErrorDisplay;
use crate::plot::line_draw::{draw_percentage_plots, PlotOpts};
use plotters::drawing::IntoDrawingArea;
use plotters::style::RGBColor;
use plotters_canvas::CanvasBackend;
use skatt_lib::calculate::CalculationRange;
use skatt_lib::generated::readme::{KVAR_EFTER_SKATT, TOTAL_PERSONLIG_SKATT, TOTAL_SKATTESUMMA};
use skatt_lib::number::f64_to_u32;
use web_sys::HtmlCanvasElement;
use yew::{html, Component, Context, Html, NodeRef, Properties};

pub struct LinePercentagePlot {
    canvas: NodeRef,
    err: Option<String>,
    plot_opts: PlotOpts,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PlotProps {
    pub calculation_range: CalculationRange,
    pub base_calculation_data: BaseCalculationData,
}

impl LinePercentagePlot {
    fn draw(&self, props: &PlotProps) -> Result<(), String> {
        let element: HtmlCanvasElement = self
            .canvas
            .cast()
            .ok_or_else(|| "Failed to cast to html canvas".to_string())?;

        let rect = element.get_bounding_client_rect();
        element.set_height(f64_to_u32(rect.height())?);
        element.set_width(f64_to_u32(rect.width())?);
        let backend = CanvasBackend::with_canvas_object(element)
            .ok_or_else(|| "Failed to create canvas backend".to_string())?;

        let drawing_area = backend.into_drawing_area();
        drawing_area
            .fill(&RGBColor(200, 200, 200))
            .map_err(|e| format!("Failed to fill drawing area: {e}"))?;

        draw_percentage_plots(
            &drawing_area,
            &props.calculation_range,
            &props.base_calculation_data,
            self.plot_opts,
        )
    }
}

pub enum PlotMessage {
    DrawErr(String),
    UpdatePersonal(Result<bool, String>),
    UpdateTotal(Result<bool, String>),
    UpdateLeft(Result<bool, String>),
}

impl Component for LinePercentagePlot {
    type Message = PlotMessage;
    type Properties = PlotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas: NodeRef::default(),
            err: None,
            plot_opts: PlotOpts {
                personal: true,
                total: true,
                left: false,
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlotMessage::UpdatePersonal(set) => match set {
                Ok(v) => {
                    self.plot_opts.personal = v;
                }
                Err(e) => {
                    self.err = Some(format!("Failed to update personal check: {e}"));
                }
            },
            PlotMessage::UpdateTotal(set) => match set {
                Ok(v) => {
                    self.plot_opts.total = v;
                }
                Err(e) => {
                    self.err = Some(format!("Failed to update total check: {e}"));
                }
            },
            PlotMessage::UpdateLeft(set) => match set {
                Ok(v) => {
                    self.plot_opts.left = v;
                }
                Err(e) => {
                    self.err = Some(format!("Failed to update left check: {e}"));
                }
            },
            PlotMessage::DrawErr(e) => {
                self.err = Some(e);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_personal = ctx.link().callback(Self::Message::UpdatePersonal);
        let on_change_total = ctx.link().callback(Self::Message::UpdateTotal);
        let on_change_left = ctx.link().callback(Self::Message::UpdateLeft);
        html! {
            <div>
                <Checkbox info_html={TOTAL_PERSONLIG_SKATT} checked={self.plot_opts.personal} on_change={on_change_personal} label_text={"Personlig skatt %"} input_name={"personal_tax_percentage"} input_id={"personal_tax_percentage"} />
                <Checkbox info_html={TOTAL_SKATTESUMMA} checked={self.plot_opts.total} on_change={on_change_total} label_text={"Totala skatter %"} input_name={"total_tax_percentage"} input_id={"total_tax_percentage"} />
                <Checkbox info_html={KVAR_EFTER_SKATT} checked={self.plot_opts.left} on_change={on_change_left} label_text={"Kvar efter skatt"} input_name={"money_left_percentage"} input_id={"money_left_percentage"} />
                <SimpleErrorDisplay message={self.err.clone()}/>
                <canvas width={"800px"} height={"600px"} ref = {self.canvas.clone()}/>
            </div>
        }
    }

    // Have to bind to the canvas before drawing
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Err(e) = self.draw(ctx.props()) {
            ctx.link().send_message(Self::Message::DrawErr(e));
        }
    }
}
