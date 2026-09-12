use thiserror::Error;

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("ObjectStore error: {0}")]
    ObjectStoreError(#[from] object_store::Error),
}
