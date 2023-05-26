pub mod graph;

use gloo_console::log;
use graph::NetworkGraph;
use graph::NodeData;
use petgraph::adj::IndexType;
use petgraph::adj::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, OffscreenCanvas};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::graph::CompanyData;

fn is_node_clicked(node_position: &(f64, f64), click_position: &(f64, f64), radius: f64) -> bool {
    let dx = node_position.0 - click_position.0;
    let dy = node_position.1 - click_position.1;
    let distance_squared = dx * dx + dy * dy;
    distance_squared <= radius * radius
}

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/:title")]
    ShowNode { title: String },
}

const BACKGROUND_COLOR: &str = "#354343";
pub struct CanvasApp {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    graph: Option<NetworkGraph<CompanyData>>,
}

impl CanvasApp {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Self { context, canvas, graph: None })
    }

    pub fn draw<A: NodeData>(mut self, graph: &NetworkGraph<A>, node: &str) -> CanvasApp {
        log!("drawing");
        // Create offscreen canvas at 2x the size than the current canvas
        let offscreen_canvas =
            OffscreenCanvas::new(self.canvas.width() * 2, self.canvas.height() * 2).unwrap();
        let offscreen_context = offscreen_canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into::<CanvasRenderingContext2d>();

        offscreen_context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
        offscreen_context.set_fill_style(&JsValue::from_str(BACKGROUND_COLOR));
        offscreen_context.fill_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
        offscreen_context.set_line_width(2.0);
        offscreen_context.set_stroke_style(&JsValue::from_str("white"));

        // Draw edges
        offscreen_context.set_stroke_style(&JsValue::from_str("white"));
        offscreen_context.set_line_width(6.0);

        let root = graph
            .node_references()
            .find(|n| n.1.title() == node)
            .unwrap();

        // create new graph:
        let mut new_graph = NetworkGraph::new();
        let width = 1800.0;
        let height = 900.0;

        // render node at the center
        let center_company = CompanyData::new(root.1.title(), width / 2.0, height / 2.0);
        new_graph.add_node(center_company);
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = 300.0;
        let mut angle: f32 = 0.0;


        {
            let area_count = graph.edges(root.0).count();
            let angle_increment = 2.0 * std::f32::consts::PI / area_count as f32;
            for edge in graph.edges(root.0) {
                let target_index = edge.target();
                let target_position = graph.node_weight(target_index).unwrap();
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                let area = CompanyData::new(target_position.title(), x, y);
                new_graph.add_node(area);
                angle += angle_increment;
            }
            for i in 0..area_count + 1 {
                let root = 0; // Assuming the root node is the first node.
                new_graph.add_edge(NodeIndex::new(root), NodeIndex::new(i), 1);
            }
        }

        let radius = 90.0 as f64;
        let root = new_graph
            .node_references()
            .find(|n| n.1.title() == node)
            .unwrap();
        for edge in new_graph.edges(root.0) {
            let source_index = edge.source();
            let target_index = edge.target();
            // Get the positions of the source and target nodes
            let source_position = new_graph.node_weight(source_index).unwrap();
            let target_position = new_graph.node_weight(target_index).unwrap();

            // Draw the edge as a line between the source and target nodes
            offscreen_context.begin_path();
            offscreen_context.move_to(source_position.x() as f64, source_position.y() as f64);
            offscreen_context.line_to(target_position.x() as f64, target_position.y() as f64);
            offscreen_context.stroke();

            let x = target_position.x() as f64;
            let y = target_position.y() as f64;
            // // Draw node
            offscreen_context.set_fill_style(&JsValue::from_str("white"));
            offscreen_context.begin_path();
            offscreen_context
                .arc(x as f64, y as f64, radius.into(), 0.0, 2.0 * std::f64::consts::PI)
                .unwrap();
            offscreen_context.fill();

            // // Draw node label
            offscreen_context.set_fill_style(&JsValue::from_str("black"));
            offscreen_context.set_font("bold 20px sans-serif");
            let text = target_position.title();
            let metrics = offscreen_context.measure_text(&text).unwrap();
            let text_width = metrics.width();
            let text_height = 20.0;
            let x = x - (text_width / 2.0);
            let y = y + (text_height / 2.0);
            offscreen_context.fill_text(&text, x, y).unwrap();
        }
        self.context
            .draw_image_with_offscreen_canvas(&offscreen_canvas, 0.0, 0.0)
            .unwrap();

        Self {
            context: self.context,
            canvas: self.canvas,
            graph: Some(new_graph)}
    }
}
