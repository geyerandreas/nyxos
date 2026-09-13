use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub data_directory: String,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            data_directory: String::from("./"),
        }
    }
}
