use axum::response::{IntoResponse, Response};

pub async fn get() -> impl IntoResponse {
    // Create a custom response with a custom header
    let mut response: Response<String> = axum::http::Response::new(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
        <title>fn index()</title>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
        body {
            font-family: Arial, Helvetica, sans-serif;
        }
        </style>
        </head>
        <body>
        Hello, World! from axum ! This is fn index()
        try <a href="/hello/name">/hello/name</a>
        try <a href="/static/hello.html">/static/hello.html</a>
        try <a href="/s/index.html">/s/index.html</a> with button to click.
        </body></html>
    "#
        .into(),
    );
    // Add a custom header
    response.headers_mut().insert(
        http::header::REFERER, // Replace with your custom header name
        http::header::HeaderValue::from_str("Custom-Value").unwrap(), // Replace with your custom header value
    );

    response
}
