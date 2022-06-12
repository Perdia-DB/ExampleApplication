use std::rc::Rc;

use yew::{Properties, function_component, html, classes, Html, UseStateHandle, Callback, Classes};
use yewdux::prelude::{use_store, Dispatch};

use crate::{data::{Blog, SelectedBlog}, assets::{PLUS_ICON, MINUS_ICON}};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct SidebarEntryProps {
    pub blog: Blog,
    pub classes: Classes,
}

#[function_component(SidebarEntry)]
pub fn sidebar_entry(props: &SidebarEntryProps) -> Html {
    let (_, dispatch) = use_store::<SelectedBlog>();
    static mut S_PROPS: Option<SidebarEntryProps> = None;
    unsafe { 
        S_PROPS = Some(props.clone());
        let onclick = dispatch.reduce_mut_callback(|state| state.blog = Some(S_PROPS.clone().unwrap().blog));
        
        html!{<div class={S_PROPS.clone().unwrap().classes} {onclick}><span>{props.blog.title.to_string()}</span></div>}
    }
        
}

#[derive(Properties, PartialEq, Clone)]
pub struct SidebarProps {
    pub data: Box<UseStateHandle<Vec<Blog>>>,
    pub selected: Rc<SelectedBlog>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    
    let plus_icon = gloo_utils::document().create_element("svg").unwrap();
    plus_icon.set_inner_html(PLUS_ICON);
    let minus_icon = gloo_utils::document().create_element("svg").unwrap();
    minus_icon.set_inner_html(MINUS_ICON);
    
    html! {
        <div class={classes!("sidebar")}>
            <div class={classes!("sidebarHeading")}>
                <h1>{"Blog-Spring"}</h1>
            </div>
            <div class={classes!("sidebarContent")}>
                {
                    props.data.iter().map(|blog| {
                        let classes = match &props.selected.blog {
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
            </div>
            <div class={classes!("sidebarControls")}>
                <div class={classes!("addBlog", "controlButton")}>
                    {Html::VRef(plus_icon.into())}
                </div>
                <div class={classes!("removeBlog", "controlButton")}>
                    {Html::VRef(minus_icon.into())}
                </div>
            </div>
        </div>
    }
}