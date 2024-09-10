mod app;
mod check;
mod error;
mod info;
pub mod input;
mod number;
pub mod plot;
mod range_picker;
mod select;
mod slider;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
