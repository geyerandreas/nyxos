use crate::error::{Result, StorageError};

use std::fs::DirBuilder;

use bytes::Bytes;
use futures::StreamExt;
use object_store::{ObjectStore, ObjectStoreExt, PutMode, local::LocalFileSystem, path::Path};

pub struct FileStorage(LocalFileSystem);

impl FileStorage {
    pub fn new(folder: &str) -> Result<Self> {
        let path = std::path::Path::new(folder);
        if !path.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(folder)
                .map_err(|e| StorageError::CreateBinPath(path.to_path_buf(), e))?;
        }
        Ok(Self(LocalFileSystem::new_with_prefix(folder)?))
    }

    pub async fn put(&self, key: &str, data: impl Into<Bytes>) -> Result<()> {
        self.storage()
            .put_opts(&Path::from(key), data.into().into(), PutMode::Create.into())
            .await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Bytes> {
        let path = Path::from(key);
        let result = self.storage().get(&path).await?;
        let bytes = result.bytes().await?;
        Ok(bytes)
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let path = Path::from(key);
        self.storage().delete(&path).await?;
        Ok(())
    }

    pub async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>> {
        let prefix_path = prefix.map(Path::from);
        let mut stream = self.storage().list(prefix_path.as_ref());
        let mut out = Vec::new();

        while let Some(meta) = stream.next().await {
            let meta = meta?;
            out.push(meta.location.to_string());
        }

        Ok(out)
    }

    pub async fn exist(&self, key: &str) -> Result<bool> {
        self.storage()
            .head(&Path::from(key))
            .await
            .map(|_| true)
            .or_else(|e| match e {
                object_store::Error::NotFound { .. } => Ok(false),
                _ => Err(e.into()),
            })
    }

    fn storage(&self) -> &LocalFileSystem {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_storage() -> (tempfile::TempDir, FileStorage) {
        let dir = tempfile::tempdir().unwrap();
        let storage = FileStorage::new(dir.path().to_str().unwrap()).unwrap();
        (dir, storage)
    }

    #[tokio::test]
    async fn put_then_get_roundtrips() {
        let (_dir, storage) = tmp_storage();
        storage.put("a/b.txt", b"Hello".to_vec()).await.unwrap();

        let data = storage.get("a/b.txt").await.unwrap();
        assert_eq!(data, Bytes::from_static(b"Hello"));
    }

    #[tokio::test]
    async fn missing_file_raises_error() {
        let (_dir, storage) = tmp_storage();
        let error = storage.get("missing.txt").await.unwrap_err();
        assert!(error.to_string().starts_with("ObjectStore"))
    }

    #[tokio::test]
    async fn put_fails_if_key_already_exists() {
        let (_dir, storage) = tmp_storage();
        storage.put("a/b.txt", b"first".to_vec()).await.unwrap();

        let result = storage.put("a/b.txt", b"second".to_vec()).await;
        assert!(result.is_err());
    }
}
