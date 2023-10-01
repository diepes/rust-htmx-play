
use axum::{
    extract::Path,
    extract::Query,
    //extract::State,
    // http::StatusCode,
    response::IntoResponse, //, Redirect}, // Response},
    // routing::{get, get_service, post},
    // Json,
    // Router,
    // ServiceExt,
};
use serde::Deserialize; 


pub async fn counter(Path(current): axum::extract::Path<usize>) -> impl IntoResponse {
    let r = current;
    eprintln!("fn counter:{}=>{}", r, r + 1);
    (r + 1).to_string()
}
#[derive(Deserialize, Debug)]
pub struct MyQuery {
    count: Option<u32>,
}
pub async fn handler(
    path: axum::extract::Path<String>,
    query: Query<MyQuery>,
    // `Method` and `HeaderMap` don't consume the request body so they can
    // put anywhere in the argument list (but before `body`)
    method: http::Method,
    headers: http::HeaderMap,
    //query: Query<Map<String, String>>,
    // `State` is also an extractor so it needs to be before `body`
    // State(state): State<State<usize>>,
    // `String` consumes the request body and thus must be the last extractor
    body: String,
) -> impl IntoResponse {
    // ...
    println!();
    log::info!("handler headers:{headers:?}");
    println!("handler method:{method}");
    println!("handler path:{path:?}");
    println!("handler query:{query:?}");
    println!("handler body:\"{body}\"");
    format!("axum lame msg from fn hander {}", query.count.unwrap())
}

pub async fn hello(name: axum::extract::Path<String>) -> impl IntoResponse {
    eprintln!("fn hello:{:?}", name);
    let n: String = name.to_string();
    format!("Hello {}!\n", n)
}
