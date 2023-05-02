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

    // pub fn drill(&self, node: )

    pub fn draw<A: NodeData>(&self, graph: &NetworkGraph<A>) {
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

        let radius = 90.0 as f64;

        // Draw edges
        offscreen_context.set_stroke_style(&JsValue::from_str("white"));
        offscreen_context.set_line_width(6.0);

        // Assume that node 0 is the root.
        let root = graph.node_references().next().unwrap();

        for edge in graph.edges(root.0) {
            let source_index = edge.source();
            let target_index = edge.target();
            // Get the positions of the source and target nodes
            let source_position = graph.node_weight(source_index).unwrap();
            let target_position = graph.node_weight(target_index).unwrap();

            log!(
                "adding edge from {:?} to {:?}",
                source_position.title(),
                target_position.title()
            );

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
                .arc(x as f64, y as f64, radius, 0.0, 2.0 * std::f64::consts::PI)
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
    }
}
