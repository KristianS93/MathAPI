use axum::{Router, routing::get};

use super::fib_sequence_endpoint::fib;

pub trait FibSequence {
    fn setup_fib(&self) -> Self;
}

impl FibSequence for Router {
    fn setup_fib(&self) -> Self {
        self.clone().route("/fib", get(fib))
    }
}   