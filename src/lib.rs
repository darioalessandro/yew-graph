pub mod graph;

use gloo_console::log;
use graph::NetworkGraph;
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

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

    pub fn draw(&self, graph: &NetworkGraph) {
        log!("drawing");
        self.context.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        self.context.set_line_width(2.0);
        self.context.set_stroke_style(&JsValue::from_str("black"));

        let radius = 30.0 as f64;

        // Draw edges
        self.context.set_stroke_style(&JsValue::from_str("black"));
        self.context.set_line_width(2.0);

        for edge in graph.edge_references() {
            let source_index = edge.source();
            let target_index = edge.target();

            // Get the positions of the source and target nodes
            let source_position = graph.node_weight(source_index).unwrap();
            let target_position = graph.node_weight(target_index).unwrap();

            // Draw the edge as a line between the source and target nodes
            self.context.begin_path();
            self.context
                .move_to(source_position.0 as f64, source_position.1 as f64);
            self.context
                .line_to(target_position.0 as f64, target_position.1 as f64);
            self.context.stroke();
        }
        

        for node in graph.node_references() {
            let (index, (x, y)) = node;
            let x = *x as f64;
            let y = *y as f64;
            // // Draw node
            self.context.set_fill_style(&JsValue::from_str("red"));
            self.context.begin_path();
            self.context
                .arc(x as f64, y as f64, radius, 0.0, 2.0 * std::f64::consts::PI)
                .unwrap();
            self.context.fill();

            // // Draw node label
            self.context.set_fill_style(&JsValue::from_str("white"));
            self.context.set_font("16px sans-serif");
            let x = (x - (radius / 2.0)).into();
            let y = (y + (radius / 4.0)).into();
            self.context
                .fill_text(
                    &format!("{x}, {y}"),
                    x,
                    y,
                )
                .unwrap();
        }


    }
}
