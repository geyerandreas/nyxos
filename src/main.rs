use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use axum::{Router, routing::get};
use tracing::{error, info, trace};

#[tokio::main]
async fn main() {
    run_server().await;
}

async fn run_server() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address: {address}"));
    info!("The server has been started on http://{address}");

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    let shutdown_signal = async move {
        #[cfg(unix)]
        {
            let mut sigterm =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("failed to install SIGTERM handler");

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    trace!("SIGINT received, initiating graceful shutdown...");
                }
                _ = sigterm.recv() => {
                    trace!("SIGTERM received, initiating graceful shutdown...");
                }
            }
        }
        #[cfg(not(unix))]
        {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
            trace!("SIGINT received, initiating graceful shutdown...");
        }

        shutdown_clone.store(true, Ordering::SeqCst);
    };

    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal);

    if let Err(e) = server.await {
        error!("Server error: {e}");
    }

    info!("Server has shut down gracefully");
}
