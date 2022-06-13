use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use web_sys::{Window, window};
use yewdux::prelude::*;

lazy_static!{
    pub static ref HOST_LOCATION: String = window().unwrap().location().host().unwrap();
}

#[derive(Clone, PartialEq, Eq, Store)]
pub struct State {
    pub selected: Option<Blog>,
    pub blogs: Option<Vec<Blog>>,
    pub editmode: bool
}

impl Default for State {
    fn default() -> Self {
        Self { selected: Default::default(), blogs: Default::default(), editmode: false }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Blog {
    pub title: String,
    pub author: String,
    pub content: String,
    pub likes: u32,
}