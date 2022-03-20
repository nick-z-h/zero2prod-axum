use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health_check", get(health_check));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
