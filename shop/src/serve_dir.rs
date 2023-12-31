use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;
// .  Result<Response<http_body::combinators::box_body::UnsyncBoxBody<axum::body::Bytes, axum::Error>>,
pub async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    println!("file_handler uri={uri}");
    let res = get_static_file(uri.clone()).await?;
    println!("res: {:?}", res);

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse::<Uri>() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    println!("get_static_file uri={uri}");
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
    println!("get_static_file req={:?}", req);

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new("static").oneshot(req).await {
        Ok(res) => {
            println!("okay");
            Ok(res.map(boxed))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
