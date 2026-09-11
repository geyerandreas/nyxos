use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub format: LogFormat,
    pub level: LogLevel,
    pub level_web_server: LogLevel,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            format: LogFormat::Compact,
            level: LogLevel::default(),
            level_web_server: LogLevel::Warn,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Compact,
    Pretty,
    Json,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl From<LogLevel> for tracing::level_filters::LevelFilter {
    fn from(value: LogLevel) -> Self {
        Self::from_level(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Settings {
        log_format: LogFormat,
    }

    #[test]
    fn test_deserialize_log_format_compact() {
        let toml = r#"
            log_format = "compact"
        "#;

        let settings: Settings = toml::from_str(toml).unwrap();
        assert_eq!(settings.log_format, LogFormat::Compact);
    }

    #[test]
    fn test_deserialize_log_format_pretty() {
        let toml = r#"
            log_format = "pretty"
        "#;

        let settings: Settings = toml::from_str(toml).unwrap();
        assert_eq!(settings.log_format, LogFormat::Pretty);
    }

    #[test]
    fn test_deserialize_log_format_json() {
        let toml = r#"
            log_format = "json"
        "#;

        let settings: Settings = toml::from_str(toml).unwrap();
        assert_eq!(settings.log_format, LogFormat::Json);
    }

    #[test]
    fn test_deserialize_log_format_invalid() {
        let toml = r#"
            log_level = "no_log_format"
        "#;

        let settings: Result<Settings, toml::de::Error> = toml::from_str(toml);
        assert!(settings.is_err());
    }
}
