# GeckoPanda
Save and load files from local disk, Google Drive, or Amazon S3.

## Usage
```rust
// main.rs
use geckopanda::{Backend, DiskStorage};

fn main() {
    // Create the storage backend
    let storage = DiskStorage::new("storagecache").unwrap();
    // Create a new file
    let file_id = storage.create_sync("example.file").unwrap();
    // Upload data to file
    let upload_data = "example file contents".as_bytes();
    storage.update_sync(&file_id, upload_data).unwrap();
    // Download file data
    let downloaded_data = storage.get_sync(&file_id).unwrap();
    assert_eq!(upload_data, downloaded_data);
    // Delete file
    storage.delete_sync(&file_id).unwrap();
}
```
