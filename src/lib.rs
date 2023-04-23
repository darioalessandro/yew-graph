use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::{EdgeRef, IntoNodeReferences};
use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, Window};

type NetworkGraph = Graph<(f32, f32), usize>;

pub struct CanvasApp {
    context: CanvasRenderingContext2d,
}

impl CanvasApp {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Self { context })
    }

    pub fn draw(&self, graph: &CompanyGraph) {
        let context = self.context.clone();
        context.set_fill_style(&JsValue::from_str("black"));

        for node in graph.node_indices() {
            let position = graph[node].clone();

            context.begin_path();
            context
                .arc(
                    position.1 as f64,
                    position.2 as f64,
                    20.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                )
                .unwrap();
            context.fill();
        }

        context.set_stroke_style(&JsValue::from_str("gray"));
        context.set_line_width(2.0);

        for edge in graph.edge_indices() {
            let (source, target) = graph.edge_endpoints(edge).unwrap();
            let source_position = graph[source].clone();
            let target_position = graph[target].clone();

            context.begin_path();
            context.move_to(source_position.1 as f64, source_position.2 as f64);
            context.line_to(target_position.1 as f64, target_position.2 as f64);
            context.stroke();
        }
    }
}

// Company has a name str
#[derive(Clone)]
pub struct Company {
    pub name: String,
}

impl Company {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

type CompanyGraph = Graph<(Company, f32, f32), usize>;

pub fn generate_security_union_graph() -> CompanyGraph {
    let mut graph = CompanyGraph::new();
    // The first node is the root node
    let root = Company::new("Security Union".to_owned());
    graph.add_node((root, 500.0, 500.0));
    graph
}

pub fn generate_random_graph(node_count: usize, edge_count: usize) -> NetworkGraph {
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
