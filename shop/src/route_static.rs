/// in main /s nest this route
/// Static pages loaded into binary :)
///
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
    Router,
};

// Load static site into bin.
use include_dir::{include_dir, Dir};
// embedding an entire directory tree into your binary
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub fn routes() -> Router {
    Router::new()
        .route("/", get(static_path_index))
        .route("/*path", get(static_path))
}
async fn static_path_index() -> impl IntoResponse {
    println!("s static_path_index() async fn");
    get_file("index.html")
}
async fn static_path(path: axum::extract::Path<String>) -> impl IntoResponse {
    //let path_default = path.unwrap_or(String::from("index.html"));
    println!("s static_path() async fn");
    let path = path.trim_start_matches('/');
    get_file(path)
}
fn get_file(path: &str) -> axum::response::Response {
    let mime_type = mime_guess::from_path(&path).first_or_text_plain();
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
