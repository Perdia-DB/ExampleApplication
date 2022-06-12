use yew::{Properties, function_component, html, classes, Html, UseStateHandle};

use crate::data::{Blog};

#[derive(Properties, PartialEq, Clone)]
pub struct SidebarProps {
    pub data: Box<UseStateHandle<Vec<Blog>>>
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    html! {
        <div class={classes!("sidebar")}>
            <div class={classes!("sidebarHeading")}>
                <h1>{"Blogs"}</h1>
            </div>
            <div class={classes!("sidebarContent")}>
                {
                    props.data.iter().map(|blog| {
                        html!{<div class={classes!("blogEntry")}><span>{blog.title.to_string()}</span></div>}
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}