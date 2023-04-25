use petgraph::{stable_graph::NodeIndex, Graph};
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::CanvasApp;

pub type NetworkGraph = Graph<(f32, f32), usize>;

fn generate_random_graph(node_count: usize, edge_count: usize) -> NetworkGraph {
    let mut graph = NetworkGraph::new();
    let mut rng = rand::thread_rng();

    for _ in 0..node_count {
        let x = rng.gen_range(50.0, 750.0);
        let y = rng.gen_range(50.0, 550.0);
        graph.add_node((x, y));
    }

    for _ in 0..edge_count {
        let source = rng.gen_range(0, node_count);
        let target = rng.gen_range(0, node_count);

        if source != target {
            graph.add_edge(NodeIndex::new(source), NodeIndex::new(target), 1);
        }
    }

    graph
}

pub struct GraphComponent {
    canvas_ref: NodeRef,
    graph: NetworkGraph,
}

pub enum Msg {
    Draw,
}

impl Component for GraphComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let graph = generate_random_graph(10, 15);
        Self {
            canvas_ref: NodeRef::default(),
            graph,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                    let canvas_app = CanvasApp::new(canvas).unwrap();
                    canvas_app.draw(&self.graph);
                }
            }
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <canvas ref={self.canvas_ref.clone()} width="800" height="600" />
                <button onclick={ctx.link().callback(|_| Msg::Draw)}>{"Draw"}</button>
            </>
        }
    }
}
