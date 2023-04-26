use gloo_console::log;
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
    canvas_ref: NodeRef,
    graphs: Vec<NetworkGraph>,
    current_graph: usize,
}

pub enum Msg {
    Draw,
    NodeClicked(Option<usize>),
    OnCanvasClick(MouseEvent),
}

impl GraphComponent {
    fn on_canvas_click(&self, event: MouseEvent) -> Msg {
        let click_x = event.client_x() as f64;
        let click_y = event.client_y() as f64;
        let click_position = (click_x, click_y);
        let node_radius = 20.0;
    
        let graph = &self.graphs[self.current_graph];
        for (node_index, node) in graph.node_indices().zip(graph.node_weights()) {
            let node_position = (node.0 as f64, node.1 as f64);
            if is_node_clicked(&node_position, &click_position, node_radius) {
                return Msg::NodeClicked(Some(node_index.index()));
            }
        }
        Msg::NodeClicked(None)
    }

}
impl Component for GraphComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            graphs: vec![generate_random_graph_root_at_center(10), generate_random_graph_root_at_center(15)],
            current_graph: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            Msg::Draw => {
                log!("1");
                if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                    log!("2");
                    let canvas_app = CanvasApp::new(canvas).unwrap();
                    canvas_app.draw(&self.graphs[self.current_graph]);
                }
                false
            },
            Msg::NodeClicked(clicked_node) => {
                log!("node clicked", clicked_node);
                if let Some(node_index) = clicked_node {
                    // Switch to the other graph.
                    log!("clicked node: {}", node_index);
                    self.current_graph = (self.current_graph + 1) % self.graphs.len();
                    if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                        let canvas_app = CanvasApp::new(canvas).unwrap();
                        canvas_app.draw(&self.graphs[self.current_graph]);
                    }
                }
                false
            },
            Msg::OnCanvasClick(event) => {
                log!("canvas clicked");
                self.on_canvas_click(event);
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
                <canvas 
                ref={self.canvas_ref.clone()} 
                width="1920" 
                height="1080" 
                onclick={callback}
                />
            </>
        }
    }
}
