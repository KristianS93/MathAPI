use axum::{extract::Path, response::{Html, IntoResponse}};
use super::sorting::{num_generator, Heapsort, MergeSort, InsertionSort};


pub async fn sorting(Path(num): Path<isize>) -> impl IntoResponse {
    let unsorted_nums = num_generator(num);
    let json = serde_json::to_string(&unsorted_nums);
    let result_json = match json {
        Ok(val) => val,
        Err(_) => "no values created".to_string(),
    };
    
    let hs = Heapsort(vec![7,2,3,4,5,6,1,9,8,0]).await;
    let ms = MergeSort().await;
    let is = InsertionSort().await;
    Html(format!(r#"
    <h1>Welcome to sorting</h1>
    <p>Generating {} random numbers!</p>
    <p>Unsorted numbers:</p>
    <p>{}</p>
    <h2>Results of different algorithms</h2>
    <p>HeapSort: {:?}</p>
    <p>MergeSort: {:?}</p>
    <p>InsertionSort: {:?}</p>
    "#, num, result_json, hs, ms, is))
}