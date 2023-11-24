[![Crate](https://img.shields.io/badge/crates.io-v0.2.0-cc5500?style=for-the-badge)](https://crates.io/crates/geckopanda)
[![Docs](https://img.shields.io/badge/Docs-116611?style=for-the-badge&logo=docs.rs)](https://docs.rs/geckopanda/0.2.0/geckopanda)
[![License](https://img.shields.io/badge/Unlicense-blue?style=for-the-badge&logo=unlicense&logoColor=white)](https://unlicense.org)

# GeckoPanda
Save and load files from local disk, Google Drive, or Amazon S3.
- ❌ Fast
- ❌ Smart
- ✅ Simple

This crate provides the `Storage` trait and several backends that implement it,
providing a very simple API for listing, creating, updating, and deleting files.
These operations can be done either asynchronously or syncronously (blocking).

## Usage
```rust
use geckopanda::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = LocalDiskStorage::new("./storagecache")?;
    // See examples for `GoogleDriveStorage` and `AmazonS3Storage`

    let file_id = storage.create_blocking("example.file")?;
    println!("Created file ID: {file_id}");

    let data = "example file content";
    storage.update_blocking(&file_id, data.as_bytes())?;
    println!("Uploaded data: {data:?}");

    let download_data = String::from_utf8(storage.get_blocking(&file_id)?)?;
    assert_eq!(data, &download_data);
    println!("Downloaded data: {download_data:?}");

    let total_size: u64 = storage
        .list_blocking()?
        .iter()
        .map(|metadata| metadata.size)
        .sum();
    println!("Total size: {total_size} bytes");

    storage.delete_blocking(&file_id)?;
    println!("Deleted file ID: {file_id}");
    Ok(())
}
```
