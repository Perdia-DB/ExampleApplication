use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Blog {
    pub title: String,
    pub author: String,
    pub content: String,
    pub likes: u32,
}