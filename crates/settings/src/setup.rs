use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setup {
    pub admin_name: String,
    pub admin_password: String,
    pub admin_email: String,
}

impl Default for Setup {
    fn default() -> Self {
        Self {
            admin_name: "admin".to_owned(),
            admin_password: std::env::var("NYXOS_ADMIN_PASSWORD").unwrap_or_default(),
            admin_email: "admin@example.com".to_owned(),
        }
    }
}
