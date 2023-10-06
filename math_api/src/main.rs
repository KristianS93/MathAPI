use feature::fib_sequence::fib_sequence::*;
use infrastructure::server::server::server;

#[tokio::main]
async fn main() {

    server().await;
    // println!("{}", nth_fibonacci(11));
    // println!("{:#?}", fib_sequence(5));
}
