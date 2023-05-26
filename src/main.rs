use std::rc::Rc;

use urlencoding::decode;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use yew::functional::*;
use yew::prelude::*;
use yew::prelude::*;
use yew_network_graph::graph::generate_graph;
use yew_network_graph::graph::CompanyData;
use yew_network_graph::graph::ContextData;
use yew_network_graph::graph::NetworkGraph;
use yew_network_graph::graph::{GraphComponent, Msg};
use yew_network_graph::Route;
use yew_router::prelude::*;
use gloo_console::log;

#[function_component(Main)]
fn app() -> Html {
    // Initialize graph
    let graph = use_reducer(|| ContextData {
        graph: generate_graph(),
    });
    html! {
        <>
        <ContextProvider<UseReducerHandle<ContextData>> context={graph}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)}/>
            </BrowserRouter>
        </ContextProvider<UseReducerHandle<ContextData>>>
        </>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Root => html! { <GraphComponent node={"Security Union".to_string()}/> },
        Route::ShowNode { title } => {
            let decoded = decode(&title).unwrap();
            html! { <GraphComponent node={decoded.to_string()}/>
            }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::start_app::<Main>();
}
