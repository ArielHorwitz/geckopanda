use anyhow::Result;
use geckopanda::{GoogleDriveStorage, Storage};

fn main() -> Result<()> {
    /* We use the [include_str] macro to insert client secrets directly into
    binary. Environment variables can also be used to insert secrets. */
    let client_secret = include_str!("../client_secret.json");
    let storage = GoogleDriveStorage::new(client_secret, "tokencache.json")?;
    let file_id = storage.create_sync("example.file")?;
    println!("created file id {file_id}");
    let data = "example file content";
    println!("uploading data: {data:?}");
    storage.update_sync(&file_id, data.as_bytes())?;
    let drive_data = String::from_utf8(storage.get_sync(&file_id)?)?;
    println!("downloaded data: {drive_data:?}");
    assert_eq!(data, &drive_data);
    storage.delete_sync(&file_id)?;
    println!("deleted file id {file_id}");
    Ok(())
}
