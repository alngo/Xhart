use std::ptr::null;

use axum::{
    routing::get,
    http::StatusCode,
    Router, extract::{Request, Path}
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet_name));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greet() -> (StatusCode, String) {
    (StatusCode::OK, "Hello world!".to_string())
}

async fn greet_name(Path(name): Path<String>) -> (StatusCode, String) {
    (StatusCode::OK, format!("Hello {}!", name))
}
