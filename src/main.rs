use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address: {address}"));
    let _ = axum::serve(listener, app).await;
}
