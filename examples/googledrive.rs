use anyhow::Result;
use geckopanda::prelude::*;

fn main() -> Result<()> {
    let client_secret = include_str!("../client_secret.json");
    let storage = GoogleDriveStorage::new(client_secret, "../token_cache.json")?;
    println!("Created Google Drive storage");

    let file_id = storage.create_blocking("example.file")?;
    println!("Created file ID: {file_id}");

    let data = "example file content";
    storage.update_blocking(&file_id, data.as_bytes())?;
    println!("Uploaded data: {data:?}");

    let drive_data = String::from_utf8(storage.get_blocking(&file_id)?)?;
    assert_eq!(data, &drive_data);
    println!("Downloaded data: {drive_data:?}");

    let total_size: u64 = storage.list_blocking().unwrap().iter()
        .map(|metadata| metadata.size).sum();
    println!("Total size: {total_size} bytes");

    storage.delete_blocking(&file_id)?;
    println!("Deleted file ID: {file_id}");
    Ok(())
}
