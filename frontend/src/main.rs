#![feature(async_closure)]
use std::sync::{Mutex, Arc};
use data::{Blog, SelectedBlog};
use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use wasm_bindgen_futures::{spawn_local};
use components::sidebar::Sidebar;
use components::view::View;

use wasm_bindgen::prelude::*;
use yewdux::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => {
        use crate::log;
        log(&format_args!($($t)*).to_string())
    }
}

mod components;
mod data;
mod assets;

#[function_component(App)]
fn app() -> Html {
    let res = Arc::new(Mutex::new(use_state_eq(|| Vec::new())));
    let lock = Arc::clone(&res);
    let (state, dispatch) = use_store::<SelectedBlog>();
    spawn_local(async move {
        let mutex = lock.lock().unwrap();
        let response = Request::get("http://127.0.0.1:8080/api/blog").send().await.unwrap();
        let text = response.text().await.unwrap();
        let blogs = serde_json::from_str::<Vec<Blog>>(text.as_str()).unwrap();
        if blogs.len() > 0 {
            dispatch.reduce_mut(|state| state.blog = Some(blogs.get(0).unwrap().clone()))
        }
        mutex.set(blogs);
    });
    html! {
        <>
            <Sidebar data={Box::new(res.lock().unwrap().clone())} selected={state.clone()}/>
            <View selected={state}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}