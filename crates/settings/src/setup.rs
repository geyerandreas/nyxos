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
            admin_password: "nyxos".to_owned(),
            admin_email: "admin@example.com".to_owned(),
        }
    }
}
