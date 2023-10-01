// export RUST_LOG=info; cargo watch -x check -x test -x run
extern crate env_logger;
extern crate log;
// Import your custom logger module
use anyhow::Result;
use axum::{
    http::StatusCode,
    response::Redirect, // IntoResponse, Response},
    routing::{get, get_service, post},
    // Json,
    Router,
    // ServiceExt,
};


pub async fn start(webserver_ip: &str, webserver_port: u16) -> Result<()> {
    //let s_dir = tower_http::services::ServeDir::new("static");
    // Create shared state using Arc and Mutex
    // let shared_state = Arc::new(Mutex::new(10));

    let app = Router::new()
        .route("/", get(crate::web::index::get))
        .route("/counter/:current", get(crate::web::main::counter))
        .route("/hello/:name", get(crate::web::main::hello))
        .nest("/s/", crate::web::route_static::routes())
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
        .route("/handler/*path", post(crate::web::main::handler))
        .route("/handler/*path", get(crate::web::main::handler))
        .route("/h/*path", get(crate::web::main::handler))
        // .layer(axum::AddData::layer(shared_state.clone()))
        ; // Add shared state as data

    let addr = format!("{}:{}", webserver_ip, webserver_port)
        .parse()
        .expect("Invalid IP Addr");
    // tracing::info!("listening on {}", addr);
    log::info!("🚀 bind to ... {}",addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    log::info!("🚀 Started ... http://{webserver_ip}:{webserver_port}");
    Ok(())
}


