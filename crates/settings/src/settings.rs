use serde::Serialize;

use crate::sqlite::SQLite;

#[derive(Debug, Serialize)]
pub struct Settings {
    pub database: SQLite,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: SQLite::default(),
        }
    }
}
