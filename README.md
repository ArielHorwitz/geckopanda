# GeckoPanda
Save and load files from local disk, Google Drive, or Amazon S3.

This crate provides the `Storage` trait and several backends that implement it,
providing a very simple API for listing, creating, updating, and deleting files.
These operations can be done either synchronously or asynchronously.

## Usage
```rust
use geckopanda::{LocalDiskStorage, Storage};

fn main() {
    let storage = LocalDiskStorage::new("./storagecache").unwrap();
    // See also `geckopanda::GoogleDriveStorage` and `geckopanda::S3Storage`

    let file_id = storage.create_sync("example.file").unwrap();
    println!("created file id {file_id}");

    let data = "example file content";
    storage.update_sync(&file_id, data.as_bytes()).unwrap();
    println!("uploaded data: {data:?}");

    let drive_data = String::from_utf8(storage.get_sync(&file_id).unwrap()).unwrap();
    assert_eq!(data, &drive_data);
    println!("downloaded data: {drive_data:?}");

    storage.delete_sync(&file_id).unwrap();
    println!("deleted file id {file_id}");
}
```

## Examples
```console
cargo run --example googledrive
cargo run --example s3
cargo run --example disk
```
