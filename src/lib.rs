pub mod graph;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use petgraph::graph::UnGraph;

pub struct CanvasApp {
    context: CanvasRenderingContext2d,
    scale: f64,
}

impl CanvasApp {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let scale = 1.0;

        Self { context, scale }
    }

    pub fn draw(&mut self, graph: &UnGraph<String, ()>) {
        // Clear canvas
        self.context.set_fill_style(&JsValue::from_str("white"));
        self.context.fill_rect(0.0, 0.0, 800.0, 600.0);

        // Draw nodes
        for node in graph.node_indices() {
            let position = graph.node_weight(node).unwrap();

            self.context.begin_path();
            self.context.set_fill_style(&JsValue::from_str("blue"));
            self.context.arc(position.0 as f64, position.1 as f64, 20.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            self.context.fill();
            self.context.close_path();
        }

        // Draw edges
        self.context.set_stroke_style(&JsValue::from_str("black"));
        for edge in graph.edge_indices() {
            let (source, target) = graph.edge_endpoints(edge).unwrap();
            let source_position = graph.node_weight(source).unwrap();
            let target_position = graph.node_weight(target).unwrap();

            self.context.begin_path();
            self.context.move_to(source_position.0 as f64, source_position.1 as f64);
            self.context.line_to(target_position.0 as f64, target_position.1 as f64);
            self.context.stroke();
            self.context.close_path();
        }
    }

    pub fn zoom(&mut self, x: f64, y: f64, factor: f64) {
        self.scale *= factor;
        self.context.translate(-x * (factor - 1.0), -y * (factor - 1.0)).unwrap();
        self.context.scale(factor, factor).unwrap();
    }
}
