use serde::Serialize;

use crate::{log::Log, registry::Registry, sqlite::SQLite};

#[derive(Debug, Serialize)]
pub struct Settings {
    pub database: SQLite,
    pub registry: Registry,
    pub log: Log,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: SQLite::default(),
            registry: Registry::default(),
            log: Log::default(),
        }
    }
}
