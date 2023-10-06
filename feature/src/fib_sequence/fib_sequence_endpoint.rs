use axum::response::Html;

use super::fib_sequence::nth_fibonacci;


pub async fn fib() -> String {

    let num = 77;
    let x = nth_fibonacci(num);
    format!("{} Fibbonacci number is: {}", num, x)
}