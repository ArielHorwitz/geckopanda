use crate::{ObjectMetadata, Storage};
use anyhow::Result;
use async_trait::async_trait;
use std::fs;
use std::path::{Path, PathBuf};

/// Provide a path to where the files directory.
///
/// ## Example
/// ```rust
/// use geckopanda::LocalDiskStorage;
/// let storage = LocalDiskStorage::new("storagecache").unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct LocalDiskStorage {
    root: PathBuf,
}

impl LocalDiskStorage {
    pub fn new(root_path: &str) -> Result<Self> {
        let root_path = Path::new(root_path).to_path_buf();
        if !root_path.is_dir() {
            fs::create_dir_all(&root_path)?;
        }
        Ok(Self { root: root_path })
    }
}

#[async_trait]
impl Storage for LocalDiskStorage {
    async fn list(&self) -> Result<Vec<ObjectMetadata>> {
        let contents = self.root.read_dir()?.filter_map(filter_file).collect();
        Ok(contents)
    }

    async fn create(&self, file_name: &str) -> Result<String> {
        self.update(file_name, "".as_bytes()).await?;
        Ok(file_name.to_owned())
    }

    async fn get(&self, file_id: &str) -> Result<Vec<u8>> {
        Ok(fs::read(self.root.join(file_id))?)
    }

    async fn update(&self, file_id: &str, data: &[u8]) -> Result<()> {
        Ok(fs::write(self.root.join(file_id), data)?)
    }

    async fn delete(&self, file_id: &str) -> Result<()> {
        Ok(fs::remove_file(self.root.join(file_id))?)
    }
}

fn filter_file(entry_result: std::io::Result<fs::DirEntry>) -> Option<ObjectMetadata> {
    let entry = entry_result.ok()?;
    let metadata = entry.metadata().ok()?;
    let entry_file_name = entry.file_name();
    let file_name = entry_file_name.to_string_lossy();
    Some(ObjectMetadata::new(
        &file_name,
        &file_name,
        &format!("{:?}", metadata.modified().ok()?),
        metadata.len(),
    ))
}
