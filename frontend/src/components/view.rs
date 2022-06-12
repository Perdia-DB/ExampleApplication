use std::rc::Rc;

use yew::{Properties, UseStateHandle, function_component, html, classes, Html};
use yewdux::prelude::{use_selector, use_store_value};

use crate::data::{Blog, SelectedBlog};

#[derive(Properties, PartialEq, Clone)]
pub struct ViewProps {
    pub selected: Rc<SelectedBlog>
}

#[function_component(View)]
pub fn view(props: &ViewProps) -> Html{
    console_log!("There");
    match &props.selected.blog {
        Some(blog) => {
            let content = gloo_utils::document().create_element("div").unwrap();
            content.set_inner_html(&blog.content.to_string());
            html!(
                <div class={classes!("view")}>
                    <h1>{blog.title.clone()}</h1>
                    <p>{blog.author.clone()}</p>
                    {Html::VRef(content.into())}
                </div>
            )
        },
        None => html!(
            <div class={classes!("view")}>
                
            </div>
        ) ,
    }

    
}