use axum::{http::StatusCode, routing::get, serve::Serve, Router};
use tokio::net::TcpListener;

type Server = Serve<Router, Router>;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app = Router::new().route("/health_check", get(health_check));
    Ok::<Server, std::io::Error>(axum::serve(listener, app))
}
