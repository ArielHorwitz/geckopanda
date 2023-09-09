[![Crate](https://img.shields.io/badge/crates.io-v0.1.0-cc5500?style=for-the-badge)](https://crates.io/crates/geckopanda)
[![Docs](https://img.shields.io/badge/Docs-116611?style=for-the-badge&logo=docs.rs)](https://docs.rs/geckopanda/latest/geckopanda)
[![License](https://img.shields.io/badge/Unlicense-blue?style=for-the-badge&logo=unlicense&logoColor=white)](https://unlicense.org)

# GeckoPanda
Save and load files from local disk, Google Drive, or Amazon S3.
- ❌ Fast
- ❌ Smart
- ✅ Simple

This crate provides the `Storage` trait and several backends that implement it,
providing a very simple API for listing, creating, updating, and deleting files.
These operations can be done either synchronously or asynchronously.

## Usage
```rust
use geckopanda::prelude::*;

fn main() {
    let storage = LocalDiskStorage::new("./storagecache").unwrap();
    // See also `GoogleDriveStorage` and `S3Storage`

    let file_id = storage.create_blocking("example.file").unwrap();
    println!("Created file ID: {file_id}");

    let data = "example file content";
    storage.update_blocking(&file_id, data.as_bytes()).unwrap();
    println!("Uploaded data: {data:?}");

    let drive_data = String::from_utf8(storage.get_blocking(&file_id).unwrap()).unwrap();
    assert_eq!(data, &drive_data);
    println!("Downloaded data: {drive_data:?}");

    storage.delete_blocking(&file_id).unwrap();
    println!("Deleted file ID: {file_id}");
}
```
