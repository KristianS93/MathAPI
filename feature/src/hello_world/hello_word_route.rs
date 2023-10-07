use axum::{Router, routing::get};

use super::hello_world_endpoint::index;


pub trait HelloWorld {
    fn setup_index(self) -> Self;
}

impl HelloWorld for Router {
    fn setup_index(self) -> Self {
        self.route("/", get(index))
    }
}   