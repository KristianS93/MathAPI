use axum::Router;
use feature::common::feature_extension::AddRoute;

pub async fn server() {
    let app = Router::new().setup();
    println!("Listening on http://localhost:3000/");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
