use gloo_console::log;
use petgraph::{stable_graph::NodeIndex, Graph};
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::CanvasApp;

pub type NetworkGraph = Graph<(f32, f32), usize>;

fn generate_random_graph_root_at_center(node_count: usize) -> NetworkGraph {
    let mut graph = NetworkGraph::new();
    let mut rng = rand::thread_rng();

    let width = 1800.0;
    let height = 900.0;
    // Root node is always at the center
    graph.add_node((width / 2.0, height / 2.0));

    for _ in 1..node_count {
        let x = rng.gen_range(50.0, width);
        let y = rng.gen_range(50.0, height);
        graph.add_node((x, y));
    }

    for i in 1..node_count {
        let root = 0; // Assuming the root node is the first node.
        graph.add_edge(NodeIndex::new(root), NodeIndex::new(i), 1);
    }

    graph
}

fn is_node_clicked(node_position: &(f64, f64), click_position: &(f64, f64), radius: f64) -> bool {
    let dx = node_position.0 - click_position.0;
    let dy = node_position.1 - click_position.1;
    let distance_squared = dx * dx + dy * dy;
    distance_squared <= radius * radius
}


pub struct GraphComponent {
    canvas_ref_1: NodeRef,
    canvas_ref_2: NodeRef,
    graphs: Vec<NetworkGraph>,
    current_graph: usize,
}

pub enum Msg {
    Draw,
    NodeClicked(usize),
    OnCanvasClick(MouseEvent),
}

impl GraphComponent {
    fn on_canvas_click(&self, event: MouseEvent) -> Option<Msg> {
        let click_x = event.client_x() as f64;
        let click_y = event.client_y() as f64;
        let click_position = (click_x, click_y);
        let node_radius = 30.0;
        log!("click position: ", click_x, click_y);
    
        let graph = &self.graphs[0];
        for (node_index, node) in graph.node_indices().zip(graph.node_weights()) {
            let node_position = (node.0 as f64, node.1 as f64);
            log!("node position: ", node_position.0, node_position.1);
            if is_node_clicked(&node_position, &click_position, node_radius) {
                log!("node clicked", node_index.index());
                let canvas1 = self.canvas_ref_1.cast::<HtmlCanvasElement>().unwrap();
                let canvas2 = self.canvas_ref_2.cast::<HtmlCanvasElement>().unwrap();

                let scale = 10.0; // The zoom factor.
                let duration = 1000; // The duration of the animation in milliseconds.

                canvas1.set_attribute(
                    "style",
                    &format!(
                        "position: absolute; top: 0; left: 0; transition: {}ms; transform: translate(-{}px, -{}px) scale({}); opacity: 0.5;",
                        duration,
                        (click_x - 400.0) * (scale - 1.0),
                        (click_y - 300.0) * (scale - 1.0),
                        scale
                    ),
                )
                .unwrap();
    
                canvas2.set_attribute(
                    "style",
                    &format!(
                        "position: absolute; top: 0; left: 0; transition: {}ms; scale({}); opacity: 1;",
                        duration,
                        // (click_x - 400.0) * (scale - 1.0),
                        // (click_y - 300.0) * (scale - 1.0),
                        1
                    ),
                )
                .unwrap();
                return Some(Msg::NodeClicked(node_index.index()));
            }
        }
        None
    }

}
impl Component for GraphComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref_1: NodeRef::default(),
            canvas_ref_2: NodeRef::default(),
            graphs: vec![generate_random_graph_root_at_center(10), generate_random_graph_root_at_center(15)],
            current_graph: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            Msg::Draw => {
                if let Some(canvas) = self.canvas_ref_1.cast::<HtmlCanvasElement>() {
                    let canvas_app = CanvasApp::new(canvas).unwrap();
                    canvas_app.draw(&self.graphs[0]);
                }
                if let Some(canvas) = self.canvas_ref_2.cast::<HtmlCanvasElement>() {
                    let canvas_app = CanvasApp::new(canvas).unwrap();
                    canvas_app.draw(&self.graphs[1]);
                }
                false
            },
            Msg::NodeClicked(node_index) => {
                log!("node clicked", node_index);
                log!("clicked node: {}", node_index);
                // self.current_graph = (self.current_graph + 1) % self.graphs.len();
                // if let Some(canvas) = self.canvas_ref_1.cast::<HtmlCanvasElement>() {
                //     let canvas_app = CanvasApp::new(canvas).unwrap();
                //     canvas_app.draw(&self.graphs[self.current_graph]);
                // }
                false
            },
            Msg::OnCanvasClick(event) => {
                log!("canvas clicked");
                if let Some(msg) = self.on_canvas_click(event) {
                    ctx.link().send_message(msg);
                }
                false
            },
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.link().send_message(Msg::Draw);
        let callback = ctx.link().callback(Msg::OnCanvasClick);
        html! {
            <>
            <div class="canvas-container" style="position: relative;">
                <canvas  ref={self.canvas_ref_1.clone()} width="1920" height="1080" onclick={callback} 
                 style="position: absolute; top: 0; left: 0; opacity: 1; transform-origin: center; z-index: 3;"
                />
                <canvas ref={self.canvas_ref_2.clone()} width="1920" height="1080" 
                style="position: absolute; top: 0; left: 0; opacity: 0; transform-origin: center; z-index: 2;"
                 />
            </div>
            </>
        }
    }
}
