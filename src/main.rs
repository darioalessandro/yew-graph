


use urlencoding::decode;


use yew::functional::*;
use yew::prelude::*;

use yew_network_graph::graph::generate_graph;

use yew_network_graph::graph::ContextData;

use yew_network_graph::graph::{GraphComponent};
use yew_network_graph::Route;
use yew_router::prelude::*;

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
            let decoded = decode(title).unwrap();
            html! { <GraphComponent node={decoded.to_string()}/>
            }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::start_app::<Main>();
}
