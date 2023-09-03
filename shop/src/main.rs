// cargo watch -x run
// use actix_web::{get, web, App, HttpServer, Responder};
use anyhow::Result;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router};

use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

//#[get("/")]
async fn index() -> impl IntoResponse {
    "Hello, World! from actix"
}

async fn hello(name: axum::extract::Path<String> ) -> impl IntoResponse {
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
//#[actix_web::main]
async fn start_webserver(webserver_ip: &str, webserver_port: u16) -> Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/hello/:name", get(hello));

    let addr = format!("{}:{}",webserver_ip,webserver_port).parse().expect("Invalid IP Addr");
    // tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
