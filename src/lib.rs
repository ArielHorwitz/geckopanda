#![doc = include_str!("../README.md")]

use anyhow::{Context, Result};
use async_trait::async_trait;
use tokio::runtime::Runtime;
pub mod backends;
#[cfg(feature = "crypto")]
mod crypto;
pub mod prelude;

/// Metadata of a specific file. Returned by [Storage::list].
#[derive(Clone, Debug)]
pub struct ObjectMetadata {
    pub id: String,
    pub name: String,
    pub last_modified: String,
    pub size: u64,
}

impl ObjectMetadata {
    pub fn new(id: &str, name: &str, last_modified: &str, size: u64) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            last_modified: last_modified.to_owned(),
            size,
        }
    }
}

/// A trait for implementing file management.
#[async_trait]
pub trait Storage: Sync {
    /// Get a file metadata list.
    async fn list(&self) -> Result<Vec<ObjectMetadata>>;

    /// Create a new file by name and get the id.
    async fn create(&self, file_name: &str) -> Result<String>;

    /// Get file data by id.
    async fn get(&self, file_id: &str) -> Result<Vec<u8>>;

    /// Update file data by id.
    async fn update(&self, file_id: &str, data: &[u8]) -> Result<()>;

    /// Delete a file by id.
    async fn delete(&self, file_id: &str) -> Result<()>;

    /// Get file data by id and decrypt.
    #[cfg(feature = "crypto")]
    async fn get_decrypt(&self, file_id: &str, key: &str) -> Result<Vec<u8>> {
        let ciphertext = self.get(file_id).await.context("get file")?;
        let plaintext = crate::crypto::decrypt(&ciphertext, key).context("decrypt data")?;
        Ok(plaintext)
    }

    /// Encrypt and update file data by id.
    #[cfg(feature = "crypto")]
    async fn update_encrypt(&self, file_id: &str, data: &[u8], key: &str) -> Result<()> {
        let ciphertext = crate::crypto::encrypt(data, key).context("encrypt data")?;
        self.update(file_id, &ciphertext)
            .await
            .context("update file")?;
        Ok(())
    }

    /// Blocking version of [Storage::list()].
    fn list_blocking(&self) -> Result<Vec<ObjectMetadata>> {
        Runtime::new()?.block_on(self.list())
    }

    /// Blocking version of [Storage::create()].
    fn create_blocking(&self, file_name: &str) -> Result<String> {
        Runtime::new()?.block_on(self.create(file_name))
    }

    /// Blocking version of [Storage::get()].
    fn get_blocking(&self, file_id: &str) -> Result<Vec<u8>> {
        Runtime::new()?.block_on(self.get(file_id))
    }

    /// Blocking version of [Storage::update()].
    fn update_blocking(&self, file_id: &str, data: &[u8]) -> Result<()> {
        Runtime::new()?.block_on(self.update(file_id, data))
    }

    /// Blocking version of [Storage::delete()].
    fn delete_blocking(&self, file_id: &str) -> Result<()> {
        Runtime::new()?.block_on(self.delete(file_id))
    }

    /// Blocking version of [Storage::get_decrypt()].
    #[cfg(feature = "crypto")]
    fn get_decrypt_blocking(&self, file_id: &str, key: &str) -> Result<Vec<u8>> {
        Runtime::new()?.block_on(self.get_decrypt(file_id, key))
    }

    /// Blocking version of [Storage::update_encrypt()].
    #[cfg(feature = "crypto")]
    fn update_encrypt_blocking(&self, file_id: &str, data: &[u8], key: &str) -> Result<()> {
        Runtime::new()?.block_on(self.update_encrypt(file_id, data, key))
    }
}
