use std::sync::Arc;

use bytes::Bytes;
use futures::StreamExt;
use object_store::{ObjectStore, ObjectStoreExt, local::LocalFileSystem, path::Path};

pub struct Storage {
    store: Arc<dyn ObjectStore>,
}

impl Storage {
    pub fn new_local(root_dir: &str) -> Self {
        let local = LocalFileSystem::new_with_prefix(root_dir).unwrap();
        Self {
            store: Arc::new(local),
        }
    }

    pub async fn put(&self, key: &str, data: Vec<u8>) -> object_store::Result<()> {
        let path = Path::from(key);
        self.store.put(&path, Bytes::from(data).into()).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> object_store::Result<Bytes> {
        let path = Path::from(key);
        let result = self.store.get(&path).await?;
        let bytes = result.bytes().await?;
        Ok(bytes)
    }

    pub async fn delete(&self, key: &str) -> object_store::Result<()> {
        let path = Path::from(key);
        self.store.delete(&path).await?;
        Ok(())
    }

    pub async fn list(&self, prefix: Option<&str>) -> object_store::Result<Vec<String>> {
        let prefix_path = prefix.map(Path::from);
        let mut stream = self.store.list(prefix_path.as_ref());
        let mut out = Vec::new();

        while let Some(meta) = stream.next().await {
            let meta = meta?;
            out.push(meta.location.to_string());
        }

        Ok(out)
    }
}
