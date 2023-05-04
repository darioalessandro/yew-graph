use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use yew::functional::*;
use yew::prelude::*;
use yew_network_graph::graph::{GraphComponent, Msg};
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/:title")]
    Home { title: String },
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

#[function_component(Main)]
fn app() -> Html {
    let graph = use_state(|| generate_graph());
    html! {
        <BrowserRouter>
         <Switch<Route> render={Switch::render(switch)} graph={*graph} />
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Root => html! { <GraphComponent node={"Security Union".to_string()}/> },
        Route::Home { title } => html! { <GraphComponent node={title.clone()}/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::start_app::<Main>();
}
