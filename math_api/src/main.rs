use infrastructure::server::server::server;

#[tokio::main]
async fn main() {

    server().await;
}
