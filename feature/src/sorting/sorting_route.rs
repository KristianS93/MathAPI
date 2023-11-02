use axum::{Router, routing::get};

use super::sorting_endpoint::sorting;

pub trait Sorting {
    fn setup_sorting(self) -> Self;
}

impl Sorting for Router {
    fn setup_sorting(self) -> Self {
        self.route("/sort/:num", get(sorting))
    }
}