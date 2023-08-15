use crate::{Backend, ObjectMetadata};
use anyhow::Result;
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Storage {
    root: PathBuf,
}

impl Storage {
    pub fn new(root_path: PathBuf) -> Result<Self> {
        if !root_path.is_dir() {
            fs::create_dir_all(&root_path)?;
        }
        Ok(Self { root: root_path })
    }
}

#[async_trait]
impl Backend for Storage {
    async fn list(&self) -> Result<Vec<ObjectMetadata>> {
        let contents = self.root.read_dir()?.filter_map(filter_file).collect();
        Ok(contents)
    }

    async fn get(&self, file: &str) -> Result<Vec<u8>> {
        Ok(fs::read(self.root.join(file))?)
    }

    async fn put(&self, file: &str, data: &[u8]) -> Result<()> {
        Ok(fs::write(self.root.join(file), data)?)
    }

    async fn delete(&self, file: &str) -> Result<()> {
        Ok(fs::remove_file(self.root.join(file))?)
    }
}

fn filter_file(entry_result: std::io::Result<fs::DirEntry>) -> Option<ObjectMetadata> {
    let entry = entry_result.ok()?;
    let metadata = entry.metadata().ok()?;
    Some(ObjectMetadata::new(
        &entry.file_name().to_string_lossy(),
        &format!("{:?}", metadata.modified().ok()?),
        metadata.len(),
    ))
}
