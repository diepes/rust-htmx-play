// use askama;
use axum::{
    extract::Path,
    extract::Query,
    http::StatusCode,
    response::IntoResponse, // Response},
    routing::get,           // , get_service, post},
    Router,
};
/// in main /s nest this route
/// Static pages loaded into binary :)
///
use minijinja; // jinja templatating // jinja templatating - compiled in

use serde::Deserialize; // , Serialize};
                        //use tower;

// Load static site into bin.
use include_dir::{include_dir, Dir};
// embedding an entire directory tree into your binary
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub fn routes() -> Router {
    Router::new()
        .route("/", get(static_path_index))
        // Note that /*key doesn’t match empty segments. Thus:
        .route("/*path", get(static_path))
        .route("/template/*path", get(template))
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

#[derive(Deserialize, Debug)]
struct MyQuery {
    count: Option<u32>,
}
async fn template(
    path: Path<String>,
    query: Query<MyQuery>,
    method: http::Method,
    headers: http::HeaderMap,
    body: String,
) -> impl IntoResponse {
    // ...
    let path = path.as_str();
    println!();
    println!("temmplate headers:{headers:?}");
    println!("template method:{method}");
    println!("template query:{query:?}");
    println!("template body:\"{body}\"");
    //format!("axum lame msg from fn template {}", query.count.unwrap())
    //let mime_type = mime_guess::from_path(path).first_or_text_plain();
    let mime_type = "text/html";
    log::info!("template path={path}  {mime_type}");
    println!("template path={path}  {mime_type}");
    match STATIC_DIR.get_file(path) {
        None => axum::response::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::boxed(axum::body::Empty::new()))
            .unwrap(),
        Some(file) => {
            let file_string = String::from_utf8_lossy(file.contents());
            println!(
                "template file_string.len()={} {}",
                file_string.len(),
                file_string
            );

            let mut env = minijinja::Environment::new();
            env.add_template("template.txt", &file_string).unwrap();
            let template = env.get_template("template.txt").unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(axum::body::boxed(axum::body::Full::from(
                    template
                        .render(minijinja::context! {
                            name => "Peter Lustig",
                            iterations => 1
                        })
                        .unwrap(),
                )))
                .unwrap()
        }
    }
}
