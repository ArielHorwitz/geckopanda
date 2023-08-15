use anyhow::Result;
use async_trait::async_trait;
use tokio::runtime::Runtime;
pub mod backends;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ObjectMetadata {
    pub name: String,
    pub last_modified: String,
    pub size: u64,
}

impl ObjectMetadata {
    pub fn new(name: &str, last_modified: &str, size: u64) -> Self {
        Self {
            name: name.to_owned(),
            last_modified: last_modified.to_owned(),
            size,
        }
    }
}

#[async_trait]
pub trait Backend {
    async fn list(&self) -> Result<Vec<ObjectMetadata>>;

    async fn get(&self, file: &str) -> Result<Vec<u8>>;

    async fn put(&self, file: &str, data: &[u8]) -> Result<()>;

    async fn delete(&self, file: &str) -> Result<()>;

    fn list_sync(&self) -> Result<Vec<ObjectMetadata>> {
        Runtime::new()?.block_on(self.list())
    }

    fn get_sync(&self, file: &str) -> Result<Vec<u8>> {
        Runtime::new()?.block_on(self.get(file))
    }

    fn put_sync(&self, file: &str, data: &[u8]) -> Result<()> {
        Runtime::new()?.block_on(self.put(file, data))
    }

    fn delete_sync(&self, file: &str) -> Result<()> {
        Runtime::new()?.block_on(self.delete(file))
    }
}
