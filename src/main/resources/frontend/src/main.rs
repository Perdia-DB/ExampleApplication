#![feature(async_closure)]
use std::sync::{Mutex, Arc};
use data::Blog;
use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use wasm_bindgen_futures::{spawn_local};
use components::sidebar::Sidebar;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

mod components;
mod data;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/blog/:name")]
    Post { name: String },
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <App /> },
        Route::Post { name } => html! { <App /> },
    }
}

#[derive(Properties, PartialEq, Clone)]
struct AppProps {}

#[function_component(App)]
fn app(props: &AppProps) -> Html {
    let res = Arc::new(Mutex::new(use_state_eq(|| Vec::new())));
    let lock = Arc::clone(&res);
    spawn_local(async move {
        let mutex = lock.lock().unwrap();
        let response = Request::get("http://127.0.0.1:8080/api/blog").send().await.unwrap();
        let text = response.text().await.unwrap();
        mutex.set(serde_json::from_str::<Vec<Blog>>(text.as_str()).unwrap());
    });
    html! {
        <>
            <Sidebar data={Box::new(res.lock().unwrap().clone())}/>
        </>
    }
}

#[function_component(MainApp)]
pub fn main_app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<MainApp>();
}