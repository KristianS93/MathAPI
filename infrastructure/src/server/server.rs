use axum::{Router, routing::get};
use feature::hello_world::hello_world_router::index;

pub async fn server() {
    let app = Router::new().route("/", get(index));
    let xd = route("xd", get(handler));
    println!("Listenig on http://localhost:3000/");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
