use axum::Router;
use nyxos_settings::cli::{CliResult, parse_cli};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tracing::{error, info, trace};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::openapi::ApiDoc;

mod openapi;

#[tokio::main]
async fn main() {
    match parse_cli() {
        CliResult::RunServer => run_server().await,
        CliResult::ShowHelp => {
            // Help is already printed by parse_cli()
        }
    }
}

async fn run_server() {
    tracing_subscriber::fmt::init();

    let state = AppStateData {};

    let app = create_router(state);

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

#[derive(Clone)]
struct AppStateData {}

fn create_router(state: AppStateData) -> Router {
    let (router, api) = OpenApiRouter::<AppStateData>::with_openapi(ApiDoc::openapi())
        .routes(routes!(say_hello))
        .split_for_parts();
    return router
        .with_state(state) // Serve Swagger UI at /api/docs with OpenAPI spec at /api/openapi.json
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", api));
}

#[utoipa::path(get, path = "/", responses((status = 200, description = "say hello")))]
async fn say_hello() -> &'static str {
    return "Hello, World";
}
