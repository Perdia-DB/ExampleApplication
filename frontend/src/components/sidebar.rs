use std::{rc::Rc, sync::{Arc, Mutex}};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC, percent_decode_str};
use reqwasm::http::{Request, Headers};
use wasm_bindgen_futures::spawn_local;
use yew::{Properties, function_component, html, classes, Html, UseStateHandle, Callback, Classes};
use yewdux::prelude::{use_store, Dispatch};

use crate::{data::{Blog, State, HOST_LOCATION}, assets::{PLUS_ICON, MINUS_ICON}};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct SidebarEntryProps {
    pub blog: Blog,
    pub classes: Classes,
}

#[function_component(SidebarEntry)]
pub fn sidebar_entry(props: &SidebarEntryProps) -> Html {
    let (_, dispatch) = use_store::<State>();
    let blog = Box::new(props.blog.clone());
    let onclick = dispatch.reduce_mut_callback(move |state| {
        state.selected = Some(*blog.clone());
        state.editmode = false;
    });

    let span = gloo_utils::document().create_element("span").unwrap();
    span.set_inner_html(&percent_decode_str(&props.blog.title.to_string()).decode_utf8().unwrap().to_string());
    
    html!{<div class={props.classes.clone()} {onclick}>{Html::VRef(span.into())}</div>}
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    
    let plus_icon = gloo_utils::document().create_element("div").unwrap();
    plus_icon.set_inner_html(PLUS_ICON);
    let minus_icon = gloo_utils::document().create_element("div").unwrap();
    minus_icon.set_inner_html(MINUS_ICON);

    let (state, dispatch) = use_store::<State>();

    let remove= dispatch.reduce_mut_future_callback(move |state| Box::pin(async move {
        if state.selected.is_none() {
            return
        }
        let response = Request::delete(
            format!("http://{}/api/blog?title={}", HOST_LOCATION.to_string(), utf8_percent_encode(&state.selected.as_ref().unwrap().title.to_string(), NON_ALPHANUMERIC)).as_str())
            .send().await.unwrap();
        
        if response.ok() {
            state.blogs.as_mut().unwrap().drain_filter(|blog| blog.title == state.selected.as_ref().unwrap().title);
            state.selected = None;
        }
    }));

    let editmode = dispatch.reduce_mut_callback(move |state| {
        if state.editmode {
            return
        }
        state.selected = None;
        state.editmode = true;
    });
    
    html! {
        <div class={classes!("sidebar")}>
            <div class={classes!("sidebarHeading")}>
                <h1>{"Blog-Spring"}</h1>
            </div>
            <div class={classes!("sidebarContent")}>
                {
                    if state.blogs.is_none() {
                        return html!{<></>}
                    } else {
                        state.blogs.as_ref().unwrap().iter().map(|blog| {
                            let classes = match &state.selected {
                                Some(selected) => {
                                    if selected.title == blog.title {
                                        classes!("blogEntry", "selectedEntry")
                                    } else {
                                        classes!("blogEntry")
                                    }
                                },
                                None => classes!("blogEntry"),
                            };
                            html!{<SidebarEntry blog={blog.clone()} classes={classes} />}
                        }).collect::<Html>()
                    }
                }
            </div>
            <div class={classes!("sidebarControls")}>
                <div class={classes!("addBlog", "controlButton")} onclick={editmode}>
                    {Html::VRef(plus_icon.into())}
                </div>
                <div class={classes!("removeBlog", "controlButton")} onclick={remove}>
                    {Html::VRef(minus_icon.into())}
                </div>
            </div>
        </div>
    }
}