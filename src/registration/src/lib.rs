use axum::{http::StatusCode, routing::get, serve::Serve, Router};

type Server = Serve<Router, Router>;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run() -> Result<Server, std::io::Error> {
    let app = Router::new().route("/health_check", get(health_check));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    Ok::<Server, std::io::Error>(axum::serve(listener, app))
}
