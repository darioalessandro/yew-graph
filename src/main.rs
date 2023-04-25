use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew_network_graph::graph::{GraphComponent, Msg};

fn main() {
    yew::start_app::<GraphComponent>();
}
