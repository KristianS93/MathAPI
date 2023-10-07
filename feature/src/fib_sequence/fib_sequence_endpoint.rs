use axum::{response::{Html, IntoResponse}, extract::Path};

use super::fib_sequence::nth_fibonacci;


pub async fn fib(Path(num): Path<usize>) -> impl IntoResponse {

    let x = nth_fibonacci(num);
    // format!("{} Fibbonacci number is: {}", num, x);
    Html(format!("<h1>{} Fibbonacci number is: {}</h1>", num, x))
}