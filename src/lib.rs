pub mod graph;

use gloo_console::log;
use graph::NetworkGraph;
use graph::NodeData;
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

const BACKGROUND_COLOR: &str = "#354343";
pub struct CanvasApp {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
}

impl CanvasApp {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Self { context, canvas })
    }

    pub fn draw<A: NodeData> (&self, graph: &NetworkGraph<A>) {
        log!("drawing");
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
        self.context.set_fill_style(&JsValue::from_str(BACKGROUND_COLOR));
        self.context.fill_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        self.context.set_line_width(2.0);
        self.context.set_stroke_style(&JsValue::from_str("white"));

        let radius = 90.0 as f64;

        // Draw edges
        self.context.set_stroke_style(&JsValue::from_str("white"));
        self.context.set_line_width(6.0);

        for edge in graph.edge_references() {
            let source_index = edge.source();
            let target_index = edge.target();

            // Get the positions of the source and target nodes
            let source_position = graph.node_weight(source_index).unwrap();
            let target_position = graph.node_weight(target_index).unwrap();

            log!("adding edge from {:?} to {:?}", source_position.title(), target_position.title());

            // Draw the edge as a line between the source and target nodes
            self.context.begin_path();
            self.context
                .move_to(source_position.x() as f64, source_position.y() as f64);
            self.context
                .line_to(target_position.x() as f64, target_position.y() as f64);
            self.context.stroke();
        }

        for node in graph.node_references() {
            let (index, data) = node;
            let x = data.x() as f64;
            let y = data.y() as f64;
            // // Draw node
            self.context.set_fill_style(&JsValue::from_str("white"));
            self.context.begin_path();
            self.context
                .arc(x as f64, y as f64, radius, 0.0, 2.0 * std::f64::consts::PI)
                .unwrap();
            self.context.fill();

            // // Draw node label
            self.context.set_fill_style(&JsValue::from_str("black"));
            self.context.set_font("bold 20px sans-serif");
            let text = data.title();
            let metrics = self.context.measure_text(&text).unwrap();
            let text_width = metrics.width();
            let text_height = 20.0;
            let x = x - (text_width / 2.0);
            let y = y + (text_height / 2.0);
            self.context.fill_text(&text, x, y).unwrap();
        }
    }
}
