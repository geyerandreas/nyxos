use clap::{CommandFactory, Parser, Subcommand};
use core::option::Option;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

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
    /// Configuration management commands
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    Init {},
}

pub enum CliResult {
    ShowHelp,
    RunServer(ResolvedSettings),
    InitConfig,
}

pub fn parse_cli() -> io::Result<CliResult> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Start {}) => Ok(CliResult::RunServer(ResolvedSettings {
            settings: Settings {
                database: SQLite::default(),
            },
        })),
        Some(Commands::Config {
            command: ConfigCommands::Init {},
        }) => {
            let path = cli
                .config_file
                .unwrap_or_else(|| PathBuf::from("nyxos.toml"));

            let settings = Settings {
                database: SQLite::default(),
            };

            let contents = toml::to_string_pretty(&settings).map_err(io::Error::other)?;

            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)?;

            file.write_all(contents.as_bytes())?;

            Ok(CliResult::InitConfig)
        }
        None => {
            Cli::command().print_help().ok();
            Ok(CliResult::ShowHelp)
        }
    }
}

pub struct ResolvedSettings {
    pub settings: Settings,
}
