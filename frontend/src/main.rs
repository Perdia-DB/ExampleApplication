#![feature(async_closure)]
#![feature(drain_filter)]
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use data::{Blog, State};
use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use wasm_bindgen_futures::{spawn_local};
use components::sidebar::Sidebar;
use components::view::View;

use wasm_bindgen::prelude::*;
use yewdux::prelude::*;

use crate::data::HOST_LOCATION;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => {
        use crate::log;
        log(&format_args!($($t)*).to_string())
    }
}

macro_rules! alert {
    ($($t:tt)*) => {
        use crate::alert;
        alert(&format_args!($($t)*).to_string())
    }
}

mod components;
mod data;
mod assets;

#[function_component(App)]
fn app() -> Html {
    let (_, dispatch) = use_store::<State>();
    spawn_local(async move {
        let response = Request::get(format!("http://{}/api/blog", HOST_LOCATION.to_string()).as_str()).send().await.unwrap();
        let text = response.text().await.unwrap();
        let blogs = serde_json::from_str::<Vec<Blog>>(text.as_str()).unwrap();
        dispatch.reduce_mut(|state| state.blogs = Some(blogs))
    });
    html! {
        <>
            <Sidebar/>
            <View/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}