use axum::Router;

use crate::{fib_sequence::fib_route::FibSequence, hello_world::hello_word_route::HelloWorld};

pub trait AddRoute {
    fn setup(self) -> Self;
}

impl AddRoute for Router{
    fn setup(self) -> Self {
        self.setup_fib().setup_index()
    }
}