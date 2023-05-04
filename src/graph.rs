use gloo_console::log;
use petgraph::{stable_graph::NodeIndex, Graph};
use rand::Rng;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use web_sys::window;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::CanvasApp;

pub trait NodeData {
    fn title(&self) -> String;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

pub type NetworkGraph<A> = Graph<A, usize>;

pub struct CompanyData {
    title: String,
    x: f32,
    y: f32,
}

impl CompanyData {
    fn new(title: String, x: f32, y: f32) -> Self {
        Self { title, x, y }
    }
}

impl NodeData for CompanyData {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

pub fn generate_graph() -> NetworkGraph<CompanyData> {
    let mut graph = NetworkGraph::new();
    let width = 1800.0;
    let height = 900.0;

    // render node at the center
    let center_company = CompanyData::new("Security Union".to_string(), width / 2.0, height / 2.0);
    graph.add_node(center_company);

    // render the rest of the nodes around the center node clockwise, starting from the top, with a radius of 50
    let radius = 300.0;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let mut angle: f32 = 0.0;
    let areas = vec![
        "Recruiters",
        "Data Annotation",
        "Customers",
        "ERP",
        "AV Testing",
    ];
    let recruiters = vec![
        "Recruiter 1",
        "Recruiter 2",
        "Recruiter 3",
        "Recruiter 4",
        "Recruiter 5",
    ];
    let data_annotation = vec![
        "Data Annotation 1",
        "Data Annotation 2",
        "Data Annotation 3",
        "Data Annotation 4",
        "Data Annotation 5",
        "Data Annotation 6",
    ];
    let customers = vec![
        "Customer 1",
        "Customer 2",
        "Customer 3",
        "Customer 4",
        "Customer 5",
        "Customer 6",
        "Customer 7",
        "Customer 8",
        "Customer 9",
        "Customer 10",
    ];
    // Samples of ERP systems
    let erp = vec!["ERP 1", "ERP 2", "ERP 3", "ERP 4"];

    let av_testing = vec!["AV Testing 1", "AV Testing 2", "AV Testing 3"];

    // Top level graph
    {
        let area_count = areas.len();
        let angle_increment = 2.0 * std::f32::consts::PI / area_count as f32;
        for area in areas {
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            let area = CompanyData::new(area.to_string(), x, y);
            graph.add_node(area);
            angle += angle_increment;
        }
        for i in 0..area_count + 1 {
            let root = 0; // Assuming the root node is the first node.
            graph.add_edge(NodeIndex::new(root), NodeIndex::new(i), 1);
        }
    }
    // add recruiters
    {
        let area_count = recruiters.len();
        let angle_increment = 2.0 * std::f32::consts::PI / area_count as f32;
        let recruiters_node = 2;
        for recruiter in recruiters {
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            let recruiter = CompanyData::new(recruiter.to_string(), x, y);
            let recruiter_index = graph.add_node(recruiter);
            graph.add_edge(NodeIndex::new(recruiters_node), recruiter_index, 1);
            angle += angle_increment;
        }
    }
    // add data annotation
    {
        let area_count = data_annotation.len();
        let angle_increment = 2.0 * std::f32::consts::PI / area_count as f32;
        let data_annotation_node = 3;
        for data_annotation in data_annotation {
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            let data_annotation = CompanyData::new(data_annotation.to_string(), x, y);
            let data_annotation_index = graph.add_node(data_annotation);
            graph.add_edge(
                NodeIndex::new(data_annotation_node),
                data_annotation_index,
                1,
            );
            angle += angle_increment;
        }
    }
    // add customers
    {
        let area_count = customers.len();
        let angle_increment = 2.0 * std::f32::consts::PI / area_count as f32;
        let customers_node = 4;
        for customer in customers {
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            let customer = CompanyData::new(customer.to_string(), x, y);
            let customer_index = graph.add_node(customer);
            graph.add_edge(NodeIndex::new(customers_node), customer_index, 1);
            angle += angle_increment;
        }
    }
    // add erp
    {
        let area_count = erp.len();
        let angle_increment = 2.0 * std::f32::consts::PI / area_count as f32;
        let erp_node = 5;
        for erp in erp {
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            let erp = CompanyData::new(erp.to_string(), x, y);
            let erp_index = graph.add_node(erp);
            graph.add_edge(NodeIndex::new(erp_node), erp_index, 1);
            angle += angle_increment;
        }
    }
    // add av_testing
    {
        let area_count = av_testing.len();
        let angle_increment = 2.0 * std::f32::consts::PI / area_count as f32;
        let av_testing_node = 6;
        for av_testing in av_testing {
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            let av_testing = CompanyData::new(av_testing.to_string(), x, y);
            let av_testing_index = graph.add_node(av_testing);
            graph.add_edge(NodeIndex::new(av_testing_node), av_testing_index, 1);
            angle += angle_increment;
        }
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
    graph: NetworkGraph<CompanyData>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct GraphComponentProps {
    pub node: String,
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
        let node_radius = 50.0;
        log!("click position: ", click_x, click_y);

        let graph = &self.graph;
        for (node_index, node) in graph.node_indices().zip(graph.node_weights()) {
            let node_position = (node.x() as f64, node.y() as f64);
            if is_node_clicked(&node_position, &click_position, node_radius) {
                let canvas1 = self.canvas_ref_1.cast::<HtmlCanvasElement>().unwrap();
                let scale = 10.0; // The zoom factor.
                let duration = 500; // The duration of the animation in milliseconds.
                let callback = Closure::wrap(Box::new(move || {
                    canvas1.set_attribute(
                        "style",
                        &format!(
                            "position: absolute; top: 0; left: 0; transition: {}ms; transform-origin: {}px {}px; transform: translate({}px, {}px) scale({}); opacity: 0.8;",
                            duration,
                            node_position.0,
                            node_position.1,
                            -node_position.0,
                            0, // - canvas.height() as f64 / 2.0,
                            scale
                        ),
                    )
                    .unwrap();
                }) as Box<dyn FnMut()>);
                window()
                    .unwrap()
                    .request_animation_frame(callback.as_ref().unchecked_ref())
                    .unwrap();
                callback.forget();
                return Some(Msg::NodeClicked(node_index.index()));
            }
        }
        None
    }
}
impl Component for GraphComponent {
    type Message = Msg;
    type Properties = GraphComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref_1: NodeRef::default(),
            graph: generate_graph(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            Msg::Draw => {
                if let Some(canvas) = self.canvas_ref_1.cast::<HtmlCanvasElement>() {
                    let canvas_app = CanvasApp::new(canvas).unwrap();
                    let node = ctx.props().node.clone();
                    canvas_app.draw(&self.graph, &node);
                }
                false
            }
            Msg::NodeClicked(node_index) => {
                log!("node clicked", node_index);
                log!("clicked node: {}", node_index);
                // self.current_graph = (self.current_graph + 1) % self.graphs.len();
                // if let Some(canvas) = self.canvas_ref_1.cast::<HtmlCanvasElement>() {
                //     let canvas_app = CanvasApp::new(canvas).unwrap();
                //     canvas_app.draw(&self.graphs[self.current_graph]);
                // }
                false
            }
            Msg::OnCanvasClick(event) => {
                log!("canvas clicked");
                if let Some(msg) = self.on_canvas_click(event) {
                    ctx.link().send_message(msg);
                }
                false
            }
        };
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
                <canvas  ref={self.canvas_ref_1.clone()} width="1800" height="900" onclick={callback}
                 style="position: absolute; top: 0; left: 0; opacity: 1;z-index: 3;"
                />
            </div>
            </>
        }
    }
}
