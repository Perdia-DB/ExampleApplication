use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct SelectedBlog {
    pub blog: Option<Blog>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Blog {
    pub title: String,
    pub author: String,
    pub content: String,
    pub likes: u32,
}