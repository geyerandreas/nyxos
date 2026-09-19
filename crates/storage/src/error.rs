use std::path::PathBuf;

use thiserror::Error;

pub type Result<T, E = StorageError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("ObjectStore error: {0}")]
    ObjectStoreError(#[from] object_store::Error),
    #[error("Failed to create bin path for wheel files {0:?}: {1}")]
    CreateBinPath(PathBuf, std::io::Error),
}
