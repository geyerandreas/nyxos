use nyxos_settings::cli::{CliResult, ResolvedSettings, parse_cli};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tracing::{error, info, trace};

mod openapi;
mod routes;

#[tokio::main]
async fn main() {
    let cli = parse_cli().expect("successful CLI parsing");

    match cli {
        CliResult::RunServer(resolved) => run_server(resolved).await,
        CliResult::ShowHelp => {
            // Help is already printed by parse_cli()
        }
        CliResult::InitConfig => {
            // done
        }
    }
}

async fn run_server(settings: ResolvedSettings) {
    tracing_subscriber::fmt::init();

    let _pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&settings.settings.database.db)
        .await
        .expect("successful database connect");

    let state = AppStateData {};

    let app = routes::create_router(state);

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
