// cargo watch -x run
// use actix_web::{get, web, App, HttpServer, Responder};
use anyhow::Result;
//use axum::prelude::*;  //could not find `prelude` in `axum`
use axum::{
    extract::Path,
    extract::Query,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
    //Json,
    Router,
    // ServiceExt,
};
use std::net::SocketAddr;
//use diesel::IntoSql;
use http;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::sync::{Arc, Mutex};
// use tokio::sync::Mutex as AsyncMutex;
// use tower::ServiceBuilder;
mod route_static;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let webserver_ip = "127.0.0.1";
    let webserver_port = 8080;
    println!("🚀 Started webserver on {webserver_ip}:{webserver_port}");
    let _web = start_webserver(webserver_ip, webserver_port).await;
    println!("Next step ..");
    Ok(())
}

async fn start_webserver(webserver_ip: &str, webserver_port: u16) -> Result<()> {
    //let s_dir = tower_http::services::ServeDir::new("static");
    // Create shared state using Arc and Mutex
    let shared_state = Arc::new(Mutex::new(10));

    let app = Router::new()
        .route("/", get(index))
        .route("/counter/:current", get(counter))
        .route("/hello/:name", get(hello))
        .nest("/s", route_static::routes())
        //.nest("/s/", route_static::routes())
        //.route("/s/", get(static_path))
        //.route("/s/*path", get(static_path))
        .nest_service(
            "/static",
            get_service(tower_http::services::ServeDir::new("static")
            .append_index_html_on_directories(true)
            .fallback(tower_http::services::ServeFile::new("static/not_found.html"))
        )
            .handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .route("/handler/*path", post(handler))
        .route("/handler/*path", get(handler))
        // .layer(axum::AddData::layer(shared_state.clone()))
        ; // Add shared state as data

    let addr = format!("{}:{}", webserver_ip, webserver_port)
        .parse()
        .expect("Invalid IP Addr");
    // tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn index() -> impl IntoResponse {
    // Create a custom response with a custom header
    let mut response: Response<String> = axum::http::Response::new(
        r"
        Hello, World! from axum !
        try /hello/name
        try /static/hello.html
    "
        .into(),
    );

    // Add a custom header
    response.headers_mut().insert(
        http::header::REFERER, // Replace with your custom header name
        http::header::HeaderValue::from_str("Custom-Value").unwrap(), // Replace with your custom header value
    );

    response
}
async fn counter(Path(current): axum::extract::Path<usize>) -> impl IntoResponse {
    let r = current;
    eprintln!("fn counter:{}=>{}", r, r + 1);
    (r + 1).to_string()
}

#[derive(Deserialize, Debug)]
struct MyQuery {
    count: Option<u32>,
}
async fn handler(
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
    println!("handler headers:{headers:?}");
    println!("handler method:{method}");
    println!("handler path:{path:?}");
    println!("handler query:{query:?}");
    println!("handler body:\"{body}\"");
    format!("lame msg from fn hander {:?}", query)
}

async fn hello(name: axum::extract::Path<String>) -> impl IntoResponse {
    eprintln!("fn hello:{:?}", name);
    let n: String = name.to_string();
    format!("Hello {}!\n", n)
}
