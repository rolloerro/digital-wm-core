use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct EchoRequest {
    message: String,
}

async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoRequest> {
    Json(payload)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/echo", post(echo));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Rust Universal Server работает на {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
