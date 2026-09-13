use clap::{CommandFactory, Parser, Subcommand};
use core::option::Option;
use std::{fs, path::PathBuf};

use crate::settings::Settings;

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
    /// Initialize a new configuration file with default values
    Init {
        /// Output file path (default: ./nyxos.toml)
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },
    /// Show the current configuration
    Show {},
}

pub enum CliResult {
    ShowHelp,
    RunServer(ResolvedSettings),
    InitConfig { output: PathBuf },
    ShowConfig(ResolvedSettings),
}

pub fn parse_cli() -> CliResult {
    let cli = Cli::parse();

    let default = ResolvedSettings {
        settings: Settings::default(),
    };

    match cli.command {
        Some(Commands::Start {}) => match cli.config_file {
            Some(config_file) => {
                let content = fs::read_to_string(config_file).unwrap();
                let toml: Settings = toml::from_str(&content).expect("lol");
                CliResult::RunServer(ResolvedSettings { settings: toml })
            }
            None => CliResult::RunServer(default),
        },
        Some(Commands::Config {
            command: ConfigCommands::Init { output },
        }) => CliResult::InitConfig {
            output: output.unwrap_or_else(|| PathBuf::from("nyxos.toml")),
        },
        Some(Commands::Config {
            command: ConfigCommands::Show {},
        }) => match cli.config_file {
            Some(config_file) => {
                let content = fs::read_to_string(config_file).unwrap();
                let toml: Settings = toml::from_str(&content).expect("lol");
                CliResult::ShowConfig(ResolvedSettings { settings: toml })
            }
            None => CliResult::ShowConfig(default),
        },
        None => {
            Cli::command().print_help().ok();
            CliResult::ShowHelp
        }
    }
}

pub struct ResolvedSettings {
    pub settings: Settings,
}
