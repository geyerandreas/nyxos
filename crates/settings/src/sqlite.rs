use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SQLite {
    pub address: String,
    pub port: u16,
    pub db: String,
    pub user: String,
    pub password: String,
}

impl Default for SQLite {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 0,
            db: "sqlite://./nyxos.sqlite?mode=rwc".to_string(),
            user: String::new(),
            password: String::new(),
        }
    }
}
