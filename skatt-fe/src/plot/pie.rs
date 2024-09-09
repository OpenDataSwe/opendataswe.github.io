use crate::app::BaseCalculationData;
use crate::error::SimpleErrorDisplay;
use crate::plot::pie_draw::draw_pie_plot;
use plotters_canvas::CanvasBackend;
use skatt_lib::number::{f64_to_u32, MaxNumberValue};
use web_sys::HtmlCanvasElement;
use yew::{html, Component, Context, Html, NodeRef, Properties};

pub struct PiePlot {
    pie_err: Option<String>,
    canvas: NodeRef,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PiePlotProps {
    pub salary: MaxNumberValue,
    pub base_calculation_data: BaseCalculationData,
}

impl PiePlot {
    fn draw(&self, props: &PiePlotProps) -> Result<(), String> {
        let element: HtmlCanvasElement = self
            .canvas
            .cast()
            .ok_or_else(|| "Expected html canvas element for pie plot".to_string())?;

        let rect = element.get_bounding_client_rect();
        element.set_height(f64_to_u32(rect.height())?);
        element.set_width(f64_to_u32(rect.width())?);
        let mut backend = CanvasBackend::with_canvas_object(element)
            .ok_or_else(|| "Failed to create pie canvas".to_string())?;
        draw_pie_plot(&mut backend, &props.base_calculation_data, props.salary)
    }
}

impl Component for PiePlot {
    type Message = String;
    type Properties = PiePlotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            pie_err: None,
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.pie_err = Some(msg);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <SimpleErrorDisplay message={self.pie_err.clone()} />
                <canvas width={"800px"} height={"600px"} ref = {self.canvas.clone()}/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Err(e) = self.draw(ctx.props()) {
            ctx.link().send_message(e);
        }
    }
}
