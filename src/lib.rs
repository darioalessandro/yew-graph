pub mod graph;

use graph::NetworkGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoNodeReferences;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

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

    pub fn draw(&self, graph: &NetworkGraph) {
        self.context.set_line_width(2.0);
        self.context.set_stroke_style(&JsValue::from_str("black"));

        let radius = 20.0 as f64;

        for node in graph.node_references() {
            let (index, (x, y)) = node;
            let x = *x as f64;
            let y = *y as f64;
            // let (x, y) = ((index.into() + 1) as f64 * 100.0, 100.0);

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
            self.context
                .fill_text(
                    &format!("{:?}", index),
                    (x - (radius / 2.0)).into(),
                    (y + (radius / 4.0)).into(),
                )
                .unwrap();
        }

        for edge in graph.edge_references() {
            // let source = edge.source();
            // let target = edge.target();
            // let (_, source_pos) = ((source.into() + 1) as f64 * 100.0, 100.0);
            // let (_, target_pos) = ((target.into() + 1) as f64 * 100.0, 100.0);

            // // Draw edge
            // self.context.begin_path();
            // // self.context.move_to(source_pos.0, source_pos.1);
            // // self.context.line_to(target_pos.0, target_pos.1);
            // self.context.stroke();
        }
    }
}
