// cargo watch -x run
// use actix_web::{get, web, App, HttpServer, Responder};
use anyhow::Result;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, post},
    Json, Router, ServiceExt,
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;

async fn index() -> impl IntoResponse {
    "Hello, World! from actix"
}

async fn hello(name: axum::extract::Path<String>) -> impl IntoResponse {
    let n: String = name.to_string();
    format!("Hello {}!\n", n)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    let app = Router::new()
        .route("/", get(index))
        .route("/hello/:name", get(hello))
        .route("/s/*path", get(static_path))
        .route(
            "/static/*path",
            get_service(tower_http::services::ServeDir::new("").fallback(
                tower_http::services::ServeFile::new("static/not_found.html"),
            ))
            .handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        );

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

use include_dir::{include_dir, Dir};
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static"); // embedding an entire directory tree into your binary
async fn static_path(path: axum::extract::Path<String>) -> impl IntoResponse {
    let mut path = path.trim_start_matches('/');
    if path.len() == 0 { path = "index.html"; }
    let mime_type = mime_guess::from_path(path).first_or_text_plain();
    println!("path={path}  {mime_type}");

    match STATIC_DIR.get_file(path) {
        None => axum::response::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::boxed(axum::body::Empty::new()))
            .unwrap(),
        Some(file) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .header(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(axum::body::boxed(axum::body::Full::from(file.contents())))
            .unwrap(),
    }
}
