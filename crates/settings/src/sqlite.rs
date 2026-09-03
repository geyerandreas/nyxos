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
            db: "nyxos.db".to_string(),
            user: String::new(),
            password: String::new(),
        }
    }
}
