use clap::{CommandFactory, Parser, Subcommand};
use core::option::Option;
use std::path::PathBuf;

use crate::{settings::Settings, sqlite::SQLite};

#[derive(Parser)]
#[command(name = "nyxos", version, about)]
pub struct Cli {
    /// Path to configuration file
    #[arg(id = "config", short = 'c', long = "config", global = true)]
    pub config_file: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {},
}

pub enum CliResult {
    ShowHelp,
    RunServer(ResolvedSettings),
}

pub fn parse_cli() -> CliResult {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Start {}) => CliResult::RunServer(ResolvedSettings {
            settings: Settings {
                database: SQLite::default(),
            },
        }),
        None => {
            Cli::command().print_help().ok();
            CliResult::ShowHelp
        }
    }
}

pub struct ResolvedSettings {
    pub settings: Settings,
}
