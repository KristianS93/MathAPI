use axum::response::Html;


pub async fn fib() -> Html<&'static str> {

    Html("<h1>Fib 11 = 89</h1>")
}