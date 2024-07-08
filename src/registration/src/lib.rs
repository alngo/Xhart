mod application; mod domain;
use axum::{
    extract,
    http::StatusCode,
    routing::{get, post},
    serve::Serve,
    Router,
};

use serde::Deserialize;
use tokio::net::TcpListener;

type Server = Serve<Router, Router>;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Deserialize)]
struct SubscriberForm {
    name: String,
    email: String,
}

async fn subscribe(extract::Json(_payload): extract::Json<SubscriberForm>) -> StatusCode {
    StatusCode::OK
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe));
    Ok::<Server, std::io::Error>(axum::serve(listener, app))
}
