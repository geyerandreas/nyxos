use axum::extract::FromRef;
use nyxos_auth::jwt::JwtService;
use nyxos_settings::{
    cli::{CliResult, ResolvedSettings, parse_cli},
    log::{Log, LogFormat},
    settings::Settings,
};
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tracing::{error, info, trace};
use tracing_subscriber::fmt::format;

mod auth;
mod openapi;
mod routes;

#[tokio::main]
async fn main() {
    let cli = parse_cli();

    match cli {
        CliResult::RunServer(resolved) => run_server(resolved).await,
        CliResult::ShowHelp => {
            // Help is already printed by parse_cli()
        }
        CliResult::InitConfig { output } => {
            init_config(&output);
        }
        CliResult::ShowConfig(resolved) => show_config(resolved),
    }
}

fn show_config(resolved: ResolvedSettings) {
    match toml::to_string_pretty(&resolved.settings) {
        Ok(toml) => println!("{toml}"),
        Err(e) => {
            eprintln!("Error serializing config: {e}");
            std::process::exit(1);
        }
    }
}

fn init_config(output: &Path) {
    if output.exists() {
        eprintln!("Error: File already exists: {}", output.display());
        eprintln!("Remove the file or specify a different output path");
        std::process::exit(1);
    }

    match toml::to_string_pretty(&Settings::default()) {
        Ok(toml) => match std::fs::write(output, toml) {
            Ok(()) => {
                println!("Configuration file created: {}", output.display());
            }
            Err(e) => {
                eprintln!("Error writing config file: {e}");
                std::process::exit(1)
            }
        },
        Err(e) => {
            eprintln!("Error serializing config: {e}");
            std::process::exit(1)
        }
    }
}

async fn run_server(settings: ResolvedSettings) {
    init_tracing(&settings.settings.log);

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&settings.settings.database.db)
        .await
        .expect("successful database connect");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("successful database migration");

    let jwt_secret = std::env::var("NYXOS_JWT_SECRET").expect("NYXOS_JWT_SECRET must be set");
    let jwt = JwtService::new(jwt_secret);

    let app = routes::create_router(AppStateData { pool, jwt });

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
struct AppStateData {
    pool: sqlx::SqlitePool,
    jwt: JwtService,
}

impl FromRef<AppStateData> for sqlx::SqlitePool {
    fn from_ref(state: &AppStateData) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppStateData> for JwtService {
    fn from_ref(state: &AppStateData) -> Self {
        state.jwt.clone()
    }
}

fn init_tracing(log: &Log) {
    let builder = tracing_subscriber::fmt().with_env_filter(format!(
        "{},mio::poll=error,want=error,sqlx::query=error,sqlx::postgres=warn,\
                sea_orm_migration=warn,cargo=error,globset=warn,\
                hyper=warn,_=warn,reqwest=warn,tower_http={},\
                object_store::aws::builder=error,h2=error",
        log.level, log.level_web_server
    ));

    match log.format {
        LogFormat::Compact => builder.event_format(format().compact()).init(),
        LogFormat::Pretty => builder.event_format(format().pretty()).init(),
        LogFormat::Json => builder.event_format(format().json()).init(),
    }
}
