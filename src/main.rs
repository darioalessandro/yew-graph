use std::rc::Rc;

use gloo_console::log;
use petgraph::Graph;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew_network_graph::{generate_security_union_graph, CanvasApp, Company};

pub struct GraphComponent {
    canvas_ref: NodeRef,
    graph: Graph<(Company, f32, f32), usize>,
}

pub enum Msg {
    Draw,
}

impl Component for GraphComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: &yew::Context<GraphComponent>) -> Self {
        let graph = generate_security_union_graph();
        Self {
            canvas_ref: NodeRef::default(),
            graph,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("Updating GraphComponent");
        match msg {
            Msg::Draw => {
                let canvas: HtmlCanvasElement =
                    self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let canvas_app = CanvasApp::new(canvas).unwrap();
                canvas_app.draw(&self.graph);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Rendering GraphComponent");
        html! {
            <>
                <canvas ref={self.canvas_ref.clone()} width="800" height="600" />
                <button onclick={ctx.link().callback(|_| Msg::Draw)}>{"Draw"}</button>
            </>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <GraphComponent />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
