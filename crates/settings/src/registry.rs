use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Registry {
    pub data_directory: PathBuf,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            data_directory: PathBuf::from("./"),
        }
    }
}
