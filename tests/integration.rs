use geckopanda::{Backend, GoogleDriveStorage};
use std::path::Path;

#[test]
fn google_drive() {
    let storage = GoogleDriveStorage::new(
        &Path::new("client_secret.json"),
        &Path::new("tokencache.json"),
        &vec!["https://www.googleapis.com/auth/drive.file"],
    ).unwrap();
    let data = "manual test file content".as_bytes();
    let file_id = storage.create_sync("manual_test_file").unwrap();
    storage.update_sync(&file_id, data).unwrap();
    let drive_data = storage.get_sync(&file_id).unwrap();
    assert_eq!(data, drive_data);
    storage.delete_sync(&file_id).unwrap();
}
