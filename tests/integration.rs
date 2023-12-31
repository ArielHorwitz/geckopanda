use geckopanda::prelude::*;
use std::fs::read_to_string;

#[test]
#[cfg(feature = "localdisk")]
fn localdisk() {
    let storage = LocalDiskStorage::new("storagecache").unwrap();
    test_storage(storage);
}

#[test]
#[cfg(feature = "google-drive")]
fn google_drive() {
    let client_secret = read_to_string("client_secret.json").unwrap();
    let storage = GoogleDriveStorage::new(client_secret, "token_cache.json").unwrap();
    test_storage(storage);
}

#[test]
#[cfg(feature = "amazon-s3")]
fn amazon_s3() {
    let config_data = read_to_string("s3config.toml").unwrap();
    let storage = AmazonS3Storage::new(config_data).unwrap();
    test_storage(storage);
}

fn test_storage(storage: impl Storage) {
    let file_count = storage.list_blocking().unwrap().len();
    let file_id = storage.create_blocking("test.file").unwrap();
    assert_eq!(file_count + 1, storage.list_blocking().unwrap().len());
    let data = "test file content".as_bytes();
    storage.update_blocking(&file_id, data).unwrap();
    let drive_data = storage.get_blocking(&file_id).unwrap();
    assert_eq!(data, drive_data);
    assert_eq!(file_count + 1, storage.list_blocking().unwrap().len());
    storage.delete_blocking(&file_id).unwrap();
    assert_eq!(file_count, storage.list_blocking().unwrap().len());
}
