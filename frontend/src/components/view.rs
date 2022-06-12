use std::rc::Rc;

use reqwasm::http::Request;
use yew::{Properties, UseStateHandle, function_component, html, classes, Html};
use yewdux::prelude::{use_selector, use_store_value, use_store};

use crate::{data::{Blog, State, HOST_LOCATION}, assets::SEND_ICON};

#[function_component(View)]
pub fn view() -> Html{
    let (state, dispatch) = use_store::<State>();
    match &state.selected {
        Some(blog) => {
            let title = gloo_utils::document().create_element("h1").unwrap();
            title.set_inner_html(&blog.title.to_string());
            let author = gloo_utils::document().create_element("p").unwrap();
            author.set_inner_html(&blog.author.to_string());
            let content = gloo_utils::document().create_element("div").unwrap();
            content.set_inner_html(&blog.content.to_string());
            html!(
                <div class={classes!("view")}>
                    {Html::VRef(title.into())}
                    {Html::VRef(author.into())}
                    {Html::VRef(content.into())}
                </div>
            )
        },
        None => { 

            if !state.editmode {
                return html!(
                    <div class={classes!("view")}>
    
                    </div>
                );
            } else {
                let mock_blog = Blog {
                    title: "Example".to_string(),
                    author: "Your Name".to_string(),
                    content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lacinia at quis risus sed vulputate odio ut enim. Pellentesque habitant morbi tristique senectus et netus et.".to_string(),
                    likes: 0,
                };

                let title = gloo_utils::document().create_element("h1").unwrap();
                title.set_inner_html(&mock_blog.title.to_string());
                title.set_attribute("contenteditable", "true").unwrap();
                title.set_id("title");

                let author = gloo_utils::document().create_element("p").unwrap();
                author.set_inner_html(&mock_blog.author.to_string());
                author.set_attribute("contenteditable", "true").unwrap();
                author.set_id("author");

                let content = gloo_utils::document().create_element("div").unwrap();
                content.set_inner_html(&mock_blog.content.to_string());
                content.set_attribute("contenteditable", "true").unwrap();
                content.set_id("content");

                let send_icon = gloo_utils::document().create_element("div").unwrap();
                send_icon.set_inner_html(SEND_ICON);

                let send = dispatch.reduce_mut_future_callback(move |state| Box::pin(async move {
                    let mut title = gloo_utils::document().get_element_by_id("title").unwrap().inner_html();
                    title = title.replace("<br>", "");
                    if state.blogs.as_ref().unwrap().iter().map(|b| b.title.to_string()).collect::<Vec<String>>().contains(&title) {
                        alert!("Blog with title {:?} already exists.", title);
                        return 
                    }
                    let author = gloo_utils::document().get_element_by_id("author").unwrap().inner_html();
                    let content = gloo_utils::document().get_element_by_id("content").unwrap().inner_html();
                    let blog = Blog {
                        title, author, content, likes: 0
                    };
                    let json = serde_json::to_string(&blog).unwrap();
                    let response = Request::post(
                        format!("http://{}/api/blog", HOST_LOCATION.to_string()).as_str())
                        .body(json).header("Content-Type", "application/json")
                        .send().await.unwrap();
                    if response.ok() {
                        state.editmode = false;
                        state.blogs.as_mut().unwrap().push(blog.clone());
                        state.selected = Some(blog);
                    }
                }));

                return html!(
                    <div class={classes!("view", "edit")}>
                        {Html::VRef(title.into())}
                        {Html::VRef(author.into())}
                        {Html::VRef(content.into())}
                        <div class={classes!("send")} onclick={send}>
                            {Html::VRef(send_icon.into())}
                        </div>
                    </div>
                );
            }
            
        },
    }

    
}