use anyhow::Result;
use geckopanda::{Storage, DiskStorage, GoogleDriveStorage};

#[test]
fn storage_disk() -> Result<()> {
    let storage = DiskStorage::new("storage")?;
    test_storage(storage)
}

#[test]
fn storage_google_drive() -> Result<()> {
    let storage = GoogleDriveStorage::new("client_secret.json", "tokencache.json")?;
    test_storage(storage)
}

// TODO test s3

fn test_storage(storage: impl Storage) -> Result<()> {
    let file_count = storage.list_sync()?.len();
    let file_id = storage.create_sync("test.file")?;
    assert_eq!(file_count + 1, storage.list_sync()?.len());
    let data = "test file content".as_bytes();
    storage.update_sync(&file_id, data)?;
    let drive_data = storage.get_sync(&file_id)?;
    assert_eq!(data, drive_data);
    assert_eq!(file_count + 1, storage.list_sync()?.len());
    storage.delete_sync(&file_id)?;
    assert_eq!(file_count, storage.list_sync()?.len());
    Ok(())
}
