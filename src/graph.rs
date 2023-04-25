use crate::CanvasApp;
use petgraph::graph::UnGraph;
use wasm_bindgen::JsCast;
use yew::events::MouseEvent;
use yew::html::NodeRef;
use yew::prelude::*;

pub struct GraphComponent {
    graph: UnGraph<String, ()>,
    canvas_app: Option<CanvasApp>,
    canvas_ref: NodeRef,
}

pub enum Msg {
    Click(MouseEvent),
}

impl Component for GraphComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let graph = UnGraph::<String, ()>::from_edges(&[
            (("100", "100"), ("300", "100"), ()),
            (("300", "100"), ("400", "200"), ()),
            (("400", "200"), ("100", "100"), ()),
        ]);

        Self {
            graph,
            canvas_app: None,
            canvas_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let mut canvas_app = CanvasApp::new(canvas);
            canvas_app.draw(&self.graph);
            self.canvas_app = Some(canvas_app);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(event) => {
                if let Some(canvas_app) = &mut self.canvas_app {
                    let x = event.client_x() as f64;
                    let y = event.client_y() as f64;
                    let factor = 1.5;
                    canvas_app.zoom(x, y, factor);
                    canvas_app.draw(&self.graph);
                }
            }
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas
                ref={self.canvas_ref.clone()}
                width="800"
                height="600"
                onclick={self.link.callback(Msg::Click)}
            />
        }
    }
}

