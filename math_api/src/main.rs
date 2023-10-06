use feature::fib_sequence::fib_sequence::*;
use infrastructure::server::server::server;

#[tokio::main]
async fn main() {

    server().await;
}
