use serde::{Deserialize, Serialize};

use crate::{log::Log, oauth2::OAuth2, registry::Registry, sqlite::SQLite};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub database: SQLite,
    pub registry: Registry,
    pub log: Log,
    pub oauth2: OAuth2,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: SQLite::default(),
            registry: Registry::default(),
            log: Log::default(),
            oauth2: OAuth2::default(),
        }
    }
}
