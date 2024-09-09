use crate::error::SimpleErrorDisplay;
use web_sys::HtmlDivElement;
use yew::{html, Component, Context, Html, NodeRef, Properties};

#[derive(Clone)]
pub struct RawHTML {
    node: NodeRef,
    err: Option<String>,
}

#[derive(Properties, Debug, Clone, Eq, PartialEq)]
pub struct RawHTMLProps {
    pub inner_html: &'static str,
}

impl RawHTML {
    fn render(&self, props: &RawHTMLProps) -> Result<(), String> {
        let div: HtmlDivElement = self
            .node
            .cast()
            .ok_or_else(|| "Failed to cast to html div".to_string())?;
        div.set_inner_html(props.inner_html);
        Ok(())
    }
}

impl Component for RawHTML {
    type Message = ();
    type Properties = RawHTMLProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node: NodeRef::default(),
            err: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <SimpleErrorDisplay message={self.err.clone()}/>
                <div ref={self.node.clone()}></div>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Err(e) = self.render(ctx.props()) {
            self.err = Some(e);
        }
    }
}
