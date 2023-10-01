// export RUST_LOG=info; cargo watch -x check -x test -x run
extern crate env_logger;
extern crate log;
// Import your custom logger module
mod my_logger;

// use actix_web::{get, web, App, HttpServer, Responder};
use anyhow::Result;
//use axum::prelude::*;  //could not find `prelude` in `axum`
use axum::{
    extract::Path,
    extract::Query,
    //extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect}, // Response},
    routing::{get, get_service, post},
    //Json,
    Router,
    // ServiceExt,
};
//use std::net::SocketAddr;
//use diesel::IntoSql;
use http;
use serde::Deserialize; //, Serialize};
                        //use serde_json::Map;
                        // use std::sync::{Arc, Mutex};
                        // use tokio::sync::Mutex as AsyncMutex;
                        // use tower::ServiceBuilder;
mod route_static;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Use your custom logger.
    log::set_boxed_logger(Box::new(my_logger::MyLogger::new())).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    //env_logger::init();

    // Now you can use the log macros to log messages.
    log::debug!("This is an debug message");
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");

    println!("Hello, world!");
    let webserver_ip = "127.0.0.1";
    let webserver_port = 8080;
    println!("🚀 Started webserver on http://{webserver_ip}:{webserver_port}");
    let _web = start_webserver(webserver_ip, webserver_port).await;
    println!("Next step ..");
    Ok(())
}

async fn start_webserver(webserver_ip: &str, webserver_port: u16) -> Result<()> {
    //let s_dir = tower_http::services::ServeDir::new("static");
    // Create shared state using Arc and Mutex
    // let shared_state = Arc::new(Mutex::new(10));

    let app = Router::new()
        .route("/", get(web::index::get))
        .route("/counter/:current", get(counter))
        .route("/hello/:name", get(hello))
        .nest("/s/", route_static::routes())
        .route("/s", get(|| async { Redirect::permanent("/s/") }))
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
        .route("/h/*path", get(handler))
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
    log::info!("handler headers:{headers:?}");
    println!("handler method:{method}");
    println!("handler path:{path:?}");
    println!("handler query:{query:?}");
    println!("handler body:\"{body}\"");
    format!("axum lame msg from fn hander {}", query.count.unwrap())
}

async fn hello(name: axum::extract::Path<String>) -> impl IntoResponse {
    eprintln!("fn hello:{:?}", name);
    let n: String = name.to_string();
    format!("Hello {}!\n", n)
}
