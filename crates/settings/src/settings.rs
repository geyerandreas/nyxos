use serde::Serialize;

use crate::{registry::Registry, sqlite::SQLite};

#[derive(Debug, Serialize)]
pub struct Settings {
    pub database: SQLite,
    pub registry: Registry,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: SQLite::default(),
            registry: Registry::default(),
        }
    }
}
